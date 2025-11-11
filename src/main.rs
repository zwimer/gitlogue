mod animation;
mod config;
mod git;
mod panes;
mod syntax;
mod theme;
mod ui;
mod widgets;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use git::GitRepository;
use std::path::PathBuf;
use theme::Theme;
use ui::UI;

#[derive(Parser, Debug)]
#[command(
    name = "git-logue",
    version = "0.1.0",
    about = "A Git history screensaver - watch your code rewrite itself",
    long_about = "git-logue is a terminal-based screensaver that replays Git commits as if a ghost developer were typing each change by hand. Characters appear, vanish, and transform with natural pacing and syntax highlighting."
)]
pub struct Args {
    #[arg(
        short,
        long,
        value_name = "PATH",
        help = "Path to Git repository (defaults to current directory)"
    )]
    pub path: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "HASH",
        help = "Replay a specific commit (single-commit mode)"
    )]
    pub commit: Option<String>,

    #[arg(
        short,
        long,
        value_name = "MS",
        default_value = "30",
        help = "Typing speed in milliseconds per character"
    )]
    pub speed: u64,

    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Theme to use (overrides config file)"
    )]
    pub theme: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Theme management commands
    Theme {
        #[command(subcommand)]
        command: ThemeCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum ThemeCommands {
    /// List all available themes
    List,
}

impl Args {
    pub fn validate(&self) -> Result<PathBuf> {
        let repo_path = self.path.clone().unwrap_or_else(|| PathBuf::from("."));

        if !repo_path.exists() {
            anyhow::bail!("Path does not exist: {}", repo_path.display());
        }

        let git_dir = repo_path.join(".git");
        if !git_dir.exists() {
            anyhow::bail!(
                "Not a Git repository: {} (or any parent directories)",
                repo_path.display()
            );
        }

        repo_path
            .canonicalize()
            .context("Failed to resolve repository path")
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle subcommands
    if let Some(command) = args.command {
        match command {
            Commands::Theme { command } => match command {
                ThemeCommands::List => {
                    println!("Available themes:");
                    for theme in Theme::available_themes() {
                        println!("  - {}", theme);
                    }
                    return Ok(());
                }
            },
        }
    }

    let repo_path = args.validate()?;
    let repo = GitRepository::open(&repo_path)?;

    let is_commit_specified = args.commit.is_some();

    // Load theme: CLI argument > config file > default
    let config = Config::load()?;
    let theme_name = args.theme.as_deref().unwrap_or(&config.theme);
    let theme = Theme::load(theme_name)?;

    // Load initial commit
    let metadata = if let Some(commit_hash) = &args.commit {
        repo.get_commit(commit_hash)?
    } else {
        repo.random_commit()?
    };

    // Create UI with repository reference (for random mode) or without (for single commit mode)
    let repo_ref = if is_commit_specified {
        None
    } else {
        Some(&repo)
    };
    let mut ui = UI::new(args.speed, is_commit_specified, repo_ref, theme);
    ui.load_commit(metadata);
    ui.run()?;

    Ok(())
}
