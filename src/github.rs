use crate::contribution::GraphQLResponse;
use anyhow::{Context, Result, bail};
use std::process::Command;

fn viewer_query(year: i32) -> String {
    format!(
        r#"query {{
  viewer {{
    login
    contributionsCollection(from: "{year}-01-01T00:00:00Z", to: "{year}-12-31T23:59:59Z") {{
      contributionCalendar {{
        totalContributions
        weeks {{
          contributionDays {{
            contributionCount
            date
            contributionLevel
          }}
        }}
      }}
    }}
  }}
}}"#
    )
}

fn user_query(login: &str, year: i32) -> String {
    format!(
        r#"query {{
  user(login: "{login}") {{
    login
    contributionsCollection(from: "{year}-01-01T00:00:00Z", to: "{year}-12-31T23:59:59Z") {{
      contributionCalendar {{
        totalContributions
        weeks {{
          contributionDays {{
            contributionCount
            date
            contributionLevel
          }}
        }}
      }}
    }}
  }}
}}"#
    )
}

pub fn fetch_contributions(username: Option<&str>, year: i32) -> Result<GraphQLResponse> {
    let query = match username {
        Some(user) => user_query(user, year),
        None => viewer_query(year),
    };

    let output = Command::new("gh")
        .args([
            "api",
            "graphql",
            "--cache",
            "0",
            "-f",
            &format!("query={query}"),
        ])
        .output()
        .context("Failed to execute gh CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("gh CLI error: {stderr}");
    }

    let response: GraphQLResponse =
        serde_json::from_slice(&output.stdout).context("Failed to parse GitHub API response")?;

    Ok(response)
}
