use anyhow::{Context, Result};
use octocrab::{Octocrab, models, params::State};
use std::{
    collections::{HashMap, HashSet},
    env,
};

#[derive(Debug)]
struct Config {
    github_token: String,
    team_members: Vec<String>,
    weight_open_prs: f64,
    weight_lines: f64,
    weight_recent: f64,
    repo_owner: String,
    repo_name: String,
    pr_number: u64,
}

impl Config {
    fn from_env() -> Result<Self> {
        Ok(Config {
            github_token: env::var("INPUT_GITHUB_TOKEN").context("Missing INPUT_GITHUB_TOKEN")?,
            team_members: env::var("INPUT_TEAM_MEMBERS")
                .context("Missing INPUT_TEAM_MEMBERS")?
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            weight_open_prs: env::var("INPUT_WEIGHT_OPEN_PRS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .context("Invalid weight-open-prs")?,
            weight_lines: env::var("INPUT_WEIGHT_LINES_PER_100")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .context("Invalid weight-lines-per-100")?,
            weight_recent: env::var("INPUT_WEIGHT_RECENT_REVIEWS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .context("Invalid weight-recent-reviews")?,
            repo_owner: env::var("GITHUB_REPOSITORY_OWNER")
                .context("Missing GITHUB_REPOSITORY_OWNER")?,
            repo_name: env::var("GITHUB_REPOSITORY")
                .context("Missing GITHUB_REPOSITORY")?
                .split('/')
                .nth(1)
                .context("Invalid GITHUB_REPOSITORY format")?
                .to_string(),
            pr_number: env::var("GITHUB_EVENT_PATH")
                .ok()
                .and_then(|path| std::fs::read_to_string(path).ok())
                .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
                .and_then(|json| json["pull_request"]["number"].as_u64())
                .context("Could not extract PR number from event")?,
        })
    }
}

#[derive(Debug)]
struct ReviewerWorkload {
    open_prs_count: usize,        // How many PRs they're reviewing
    total_lines_in_review: usize, // Total lines across all PRs
}

#[derive(Debug)]
struct ReviewerScore {
    username: String,
    open_prs_count: usize,
    total_lines_in_review: usize,
    recent_reviews_count: usize,
    total_score: f64,
}

async fn fetch_recent_reviews(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    username: &str,
) -> Result<usize> {
    let since = chrono::Utc::now() - chrono::Duration::days(7);

    let search_query = format!(
        "repo:{}/{} is:pr reviewed-by:{} closed:>{}",
        owner,
        repo,
        username,
        since.format("%Y-%m-%d")
    );

    let result = octocrab
        .search()
        .issues_and_pull_requests(&search_query)
        .send()
        .await
        .context(format!("Failed to search recent reviews for @{}", username))?;

    Ok(result.total_count.unwrap_or(0) as usize)
}

async fn calculate_scores(
    octocrab: &Octocrab,
    config: &Config,
    workloads: HashMap<String, ReviewerWorkload>,
    pr_author: &str,
) -> Result<Vec<ReviewerScore>> {
    println!("🧮 Calculating scores for each reviewer...");

    let mut scores = vec![];

    for member in &config.team_members {
        // Skip the PR author
        if member == pr_author {
            continue;
        }

        let load = workloads
            .get(member)
            .expect("Member should be in the workloads");

        let recent_reviews_count =
            fetch_recent_reviews(octocrab, &config.repo_owner, &config.repo_name, member)
                .await
                .unwrap_or(0);

        // Calculate score using the weights
        let score = (load.open_prs_count as f64 * config.weight_open_prs)
            + ((load.total_lines_in_review as f64 / 100.0) * config.weight_lines)
            + (recent_reviews_count as f64 * config.weight_recent);

        println!(
            "  @{}: {:.2} points (Open: {} × {}, Lines: {} ÷ 100 × {}, Recent: {} × {})",
            member,
            score,
            load.open_prs_count,
            config.weight_open_prs,
            load.total_lines_in_review,
            config.weight_lines,
            recent_reviews_count,
            config.weight_recent
        );

        scores.push(ReviewerScore {
            username: member.clone(),
            open_prs_count: load.open_prs_count,
            total_lines_in_review: load.total_lines_in_review,
            recent_reviews_count,
            total_score: score,
        });
    }

    // Sort by score (lowest first)
    scores.sort_by(|a, b| {
        a.total_score
            .partial_cmp(&b.total_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    Ok(scores)
}

async fn fetch_open_prs_workload(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    team_members: &[String],
) -> Result<HashMap<String, ReviewerWorkload>> {
    let mut workload: HashMap<String, ReviewerWorkload> = HashMap::new();

    // Initialize workload for all team members
    for member in team_members {
        workload.insert(
            member.clone(),
            ReviewerWorkload {
                open_prs_count: 0,
                total_lines_in_review: 0,
            },
        );
    }

    let mut pr_numbers = vec![];

    let mut total_prs = 0;
    let mut page = octocrab
        .pulls(owner, repo)
        .list()
        .state(State::Open)
        .per_page(100)
        .send()
        .await
        .context("Failed to fetch open PRs")?;

    loop {
        for pr in &page {
            total_prs += 1;
            pr_numbers.push(pr.number);
        }

        page = match octocrab
            .get_page::<models::pulls::PullRequest>(&page.next)
            .await
            .context("Failed to get next page")?
        {
            Some(next_page) => next_page,
            None => break,
        }
    }

    println!("  Found {} open PRs, fetching details...", pr_numbers.len());

    for pr_number in pr_numbers {
        let pr = octocrab
            .pulls(owner, repo)
            .get(pr_number)
            .await
            .context(format!("Failed to fetch details for PR #{}", pr_number))?;

        let additions = pr.additions.unwrap_or_default();
        let deletions = pr.deletions.unwrap_or_default();
        let lines = additions + deletions;

        // Track which reviewers we've counted for this PR (to avoid double-counting)
        let mut reviewers_for_this_pr = HashSet::new();

        // Check requested reviewers (people who haven't reviewed yet)
        if let Some(requested_reviewers) = pr.requested_reviewers {
            for reviewer in requested_reviewers {
                if workload.contains_key(&reviewer.login) {
                    reviewers_for_this_pr.insert(reviewer.login.clone());
                }
            }
        }

        // Also check people who have already submitted reviews
        // (they might still be actively reviewing/assigned)
        let reviews = octocrab
            .pulls(owner, repo)
            .list_reviews(pr_number)
            .per_page(100)
            .send()
            .await
            .ok(); // Ignore errors, some PRs might not have reviews

        if let Some(reviews) = reviews {
            for review in reviews {
                if let Some(reviewer) = review.user
                    && workload.contains_key(&reviewer.login)
                {
                    reviewers_for_this_pr.insert(reviewer.login.clone());
                }
            }
        }

        for reviewer in reviewers_for_this_pr {
            if let Some(workload_entry) = workload.get_mut(&reviewer) {
                workload_entry.open_prs_count += 1;
                workload_entry.total_lines_in_review += lines as usize;

                println!(
                    "  PR #{}: @{} reviewing ({} additions, {} deletions)",
                    pr_number, reviewer, additions, deletions
                );
            }
        }
    }

    println!("\n✅ Analyzed {} open PRs", total_prs);

    Ok(workload)
}

async fn fetch_current_pr(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    pr_number: u64,
) -> Result<String> {
    println!("🔍 Fetching PR #{}...", pr_number);

    let pr = octocrab
        .pulls(owner, repo)
        .get(pr_number)
        .await
        .context("Failed to fetch PR from GitHub API")?;

    let author = pr.user.map(|u| u.login).context("PR has no author")?;

    println!("  Author: @{}", author);
    println!("  Title: {}", pr.title.as_deref().unwrap_or("(no title)"));
    println!("  State: {:?}", pr.state);

    Ok(author)
}

async fn assign_reviewer(
    octocrab: &Octocrab,
    owner: &str,
    repo: &str,
    pr_number: u64,
    reviewer: &str,
) -> Result<()> {
    println!("🔄 Assigning @{} to PR #{}...", reviewer, pr_number);

    octocrab
        .pulls(owner, repo)
        .request_reviews(pr_number, [reviewer.to_string()], [])
        .await
        .context(format!(
            "Failed to assign @{} as reviewer to PR #{}",
            reviewer, pr_number
        ))?;

    println!("✅ Successfully assigned @{} as reviewer!", reviewer);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 Parsing configuration from environment...\n");

    let config = Config::from_env()?;

    println!("✅ Configuration loaded successfully!");
    println!("\n📋 Config Details:");
    println!("  Repository: {}/{}", config.repo_owner, config.repo_name);
    println!("  PR Number: {}", config.pr_number);
    println!("  Team Members: {:?}", config.team_members);
    println!("\n⚖️  Weights:");
    println!("  Open PRs: {}", config.weight_open_prs);
    println!("  Lines per 100: {}", config.weight_lines);
    println!("  Recent reviews: {}", config.weight_recent);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔌 Connecting to GitHub API...\n");

    let octocrab = Octocrab::builder()
        .personal_token(config.github_token.clone())
        .build()
        .context("Failed to create GitHub API client")?;

    println!("✅ Connected to GitHub API");

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let pr_author = fetch_current_pr(
        &octocrab,
        &config.repo_owner,
        &config.repo_name,
        config.pr_number,
    )
    .await?;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let workload = fetch_open_prs_workload(
        &octocrab,
        &config.repo_owner,
        &config.repo_name,
        &config.team_members,
    )
    .await?;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let scores = calculate_scores(&octocrab, &config, workload, &pr_author).await?;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏆 Final Rankings (lowest score = least busy):\n");

    for (i, score) in scores.iter().enumerate() {
        let rank_emoji = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "  ",
        };

        println!(
            "{} #{} @{}: {:.2} points",
            rank_emoji,
            i + 1,
            score.username,
            score.total_score
        );
        println!(
            "       {} open PRs, {} lines, {} recent reviews",
            score.open_prs_count, score.total_lines_in_review, score.recent_reviews_count
        );
    }

    // Assign the reviewer with the lowest score
    if let Some(winner) = scores.first() {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("✨ Best choice: @{}", winner.username);

        assign_reviewer(
            &octocrab,
            &config.repo_owner,
            &config.repo_name,
            config.pr_number,
            &winner.username,
        )
        .await?;

        println!(
            "\n🎉 Done! PR #{} has been assigned to @{}",
            config.pr_number, winner.username
        );
    } else {
        println!(
            "\n⚠️  No eligible reviewers found (is everyone except the author on the team list?)"
        );
    }

    Ok(())
}
