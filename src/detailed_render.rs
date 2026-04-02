use chrono::{DateTime, Utc, NaiveDateTime};
use std::io::{self, Write};
use crate::model::{
    GroupedContributions, ContributionType, CommitContribution, IssueContribution,
    PullRequestContribution, PullRequestReviewContribution, RepositoryContribution
};

pub fn render_detailed_contributions(
    grouped: &GroupedContributions,
    from: DateTime<Utc>,
    to: DateTime<Utc>
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Header
    writeln!(handle, "Recent Contributions ({})", format_date_range(from, to))?;
    writeln!(handle, "{}", "=".repeat(50))?;
    writeln!(handle)?;

    // Summary
    let total_contributions = grouped.commits.len() + grouped.issues.len() + 
                            grouped.pull_requests.len() + grouped.pull_request_reviews.len() + 
                            grouped.repositories.len();
    
    if total_contributions == 0 {
        writeln!(handle, "No contributions found for this period.")?;
        return Ok(());
    }

    writeln!(handle, "Summary:")?;
    writeln!(handle, "  • Commits: {}", grouped.commits.len())?;
    writeln!(handle, "  • Issues: {}", grouped.issues.len())?;
    writeln!(handle, "  • Pull Requests: {}", grouped.pull_requests.len())?;
    writeln!(handle, "  • Pull Request Reviews: {}", grouped.pull_request_reviews.len())?;
    writeln!(handle, "  • Repositories: {}", grouped.repositories.len())?;
    writeln!(handle)?;

    // Render each contribution type
    render_commits(&mut handle, &grouped.commits)?;
    render_issues(&mut handle, &grouped.issues)?;
    render_pull_requests(&mut handle, &grouped.pull_requests)?;
    render_pull_request_reviews(&mut handle, &grouped.pull_request_reviews)?;
    render_repositories(&mut handle, &grouped.repositories)?;

    Ok(())
}

fn render_commits(handle: &mut io::StdoutLock, commits: &[CommitContribution]) -> io::Result<()> {
    if commits.is_empty() {
        return Ok(());
    }

    writeln!(handle, "{}", ContributionType::Commit.display_name())?;
    writeln!(handle, "{}", "-".repeat(ContributionType::Commit.display_name().len()))?;

    for commit in commits {
        let date = parse_date(&commit.occurred_at);
        writeln!(handle, "  • {} commits to {}", 
                commit.commit_count, 
                commit.repository.name_with_owner)?;
        writeln!(handle, "    {}", format_date(date))?;
    }
    writeln!(handle)?;
    Ok(())
}

fn render_issues(handle: &mut io::StdoutLock, issues: &[IssueContribution]) -> io::Result<()> {
    if issues.is_empty() {
        return Ok(());
    }

    writeln!(handle, "{}", ContributionType::Issue.display_name())?;
    writeln!(handle, "{}", "-".repeat(ContributionType::Issue.display_name().len()))?;

    for issue in issues {
        let date = parse_date(&issue.occurred_at);
        let status = if issue.issue.state == "OPEN" { "open" } else { "closed" };
        writeln!(handle, "  • {} #{}: {}", 
                issue.issue.repository.name_with_owner,
                issue.issue.number,
                issue.issue.title)?;
        writeln!(handle, "    {} • {}", format_date(date), status)?;
    }
    writeln!(handle)?;
    Ok(())
}

fn render_pull_requests(handle: &mut io::StdoutLock, prs: &[PullRequestContribution]) -> io::Result<()> {
    if prs.is_empty() {
        return Ok(());
    }

    writeln!(handle, "{}", ContributionType::PullRequest.display_name())?;
    writeln!(handle, "{}", "-".repeat(ContributionType::PullRequest.display_name().len()))?;

    for pr in prs {
        let date = parse_date(&pr.occurred_at);
        let status = if pr.pull_request.merged_at.is_some() {
            "merged"
        } else if pr.pull_request.state == "OPEN" {
            "open"
        } else {
            "closed"
        };
        writeln!(handle, "  • {} #{}: {}", 
                pr.pull_request.repository.name_with_owner,
                pr.pull_request.number,
                pr.pull_request.title)?;
        writeln!(handle, "    {} • {}", format_date(date), status)?;
    }
    writeln!(handle)?;
    Ok(())
}

fn render_pull_request_reviews(handle: &mut io::StdoutLock, reviews: &[PullRequestReviewContribution]) -> io::Result<()> {
    if reviews.is_empty() {
        return Ok(());
    }

    writeln!(handle, "{}", ContributionType::PullRequestReview.display_name())?;
    writeln!(handle, "{}", "-".repeat(ContributionType::PullRequestReview.display_name().len()))?;

    for review in reviews {
        let date = parse_date(&review.occurred_at);
        writeln!(handle, "  • Reviewed {} #{}: {}", 
                review.pull_request_review.pull_request.repository.name_with_owner,
                review.pull_request_review.pull_request.number,
                review.pull_request_review.pull_request.title)?;
        writeln!(handle, "    {} • {}", format_date(date), review.pull_request_review.state.to_lowercase())?;
    }
    writeln!(handle)?;
    Ok(())
}

fn render_repositories(handle: &mut io::StdoutLock, repos: &[RepositoryContribution]) -> io::Result<()> {
    if repos.is_empty() {
        return Ok(());
    }

    writeln!(handle, "{}", ContributionType::Repository.display_name())?;
    writeln!(handle, "{}", "-".repeat(ContributionType::Repository.display_name().len()))?;

    for repo in repos {
        let date = parse_date(&repo.occurred_at);
        let description = repo.repository.description.as_deref().unwrap_or("No description");
        writeln!(handle, "  • {}: {}", 
                repo.repository.name_with_owner,
                description)?;
        writeln!(handle, "    {}", format_date(date))?;
        
        // Show stats if available
        if let (Some(stars), Some(forks)) = (repo.repository.stargazer_count, repo.repository.fork_count) {
            if stars > 0 || forks > 0 {
                writeln!(handle, "    ⭐ {} • 🍴 {}", stars, forks)?;
            }
        }
    }
    writeln!(handle)?;
    Ok(())
}

fn parse_date(date_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| {
            // Fallback parsing
            NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S")
                .map(|ndt| DateTime::from_naive_utc_and_offset(ndt, Utc))
                .unwrap_or_else(|_| Utc::now())
        })
}

fn format_date(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let diff = now - date;
    
    if diff.num_days() == 0 {
        "today".to_string()
    } else if diff.num_days() == 1 {
        "yesterday".to_string()
    } else if diff.num_days() < 7 {
        format!("{} days ago", diff.num_days())
    } else if diff.num_days() < 30 {
        format!("{} weeks ago", diff.num_weeks())
    } else {
        date.format("%b %d").to_string()
    }
}

fn format_date_range(from: DateTime<Utc>, to: DateTime<Utc>) -> String {
    let from_str = from.format("%b %d").to_string();
    let to_str = to.format("%b %d, %Y").to_string();
    format!("{} - {}", from_str, to_str)
} 