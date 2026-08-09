pub mod Render {
    use std::{collections::HashMap, process::Command};
    use ratatui::{
        backend::CrosstermBackend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        text::{Line, Span, Text},
        widgets::{Block, Borders, Paragraph, Wrap},
        Frame,
    };
    use crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io;

    #[derive(Debug, Clone)]
    pub struct Commit {
        pub hash: String,
        pub author: String,
        pub date: String,
        pub message: String,
        pub branches: Vec<String>,
        pub parents: Vec<String>,
    }

    #[derive(Debug, Clone)]
    pub struct GraphNode {
        pub commit: Commit,
        pub column: usize,
        pub color: Color,
    }

    pub struct GitGraph {
        pub commits: Vec<GraphNode>,
        pub branch_colors: HashMap<String, Color>,
        pub selected_index: usize,
    }

    impl GitGraph {
        pub fn new() -> Self {
            Self {
                commits: Vec::new(),
                branch_colors: HashMap::new(),
                selected_index: 0,
            }
        }

        pub fn load_history(&mut self) {
            let output = Command::new("git")
                .args(["log", "--all", "--pretty=format:%H|%an|%ad|%s", "--date=short"])
                .output()
                .expect("Failed to execute git log");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut commits: Vec<Commit> = Vec::new();

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    let hash = parts[0].to_string();
                    let author = parts[1].to_string();
                    let date = parts[2].to_string();
                    let message = parts[3].to_string();

                    let branches_output = Command::new("git")
                        .args(["branch", "--contains", &hash])
                        .output()
                        .expect("Failed to get branches");
                    let branches_stdout = String::from_utf8_lossy(&branches_output.stdout);
                    let branches: Vec<String> = branches_stdout
                        .lines()
                        .map(|b| b.trim().replace('*', "").trim().to_string())
                        .filter(|b| !b.is_empty())
                        .collect();

                    let parents_output = Command::new("git")
                        .args(["rev-parse", &format!("{}^@", hash)])
                        .output()
                        .unwrap_or_else(|_| Command::new("echo").arg("").output().unwrap());
                    let parents_stdout = String::from_utf8_lossy(&parents_output.stdout);
                    let parents: Vec<String> = parents_stdout
                        .lines()
                        .map(|p| p.trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect();

                    commits.push(Commit {
                        hash,
                        author,
                        date,
                        message,
                        branches,
                        parents,
                    });
                }
            }

            self.build_graph(commits);
        }

        fn build_graph(&mut self, commits: Vec<Commit>) {
            let mut column_map: HashMap<String, usize> = HashMap::new();
            let mut next_column = 0;
            let colors = vec![
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::LightRed,
                Color::LightGreen,
                Color::LightYellow,
                Color::LightBlue,
                Color::LightMagenta,
                Color::LightCyan,
            ];
            let mut color_index = 0;

            for commit in commits {
                let column = if commit.parents.is_empty() {
                    if !column_map.contains_key(&commit.hash) {
                        column_map.insert(commit.hash.clone(), next_column);
                        next_column += 1;
                    }
                    column_map[&commit.hash]
                } else {
                    let parent_col = column_map.get(&commit.parents[0]).copied().unwrap_or(0);
                    column_map.insert(commit.hash.clone(), parent_col);
                    parent_col
                };

                let color = if !commit.branches.is_empty() {
                    for branch in &commit.branches {
                        if !self.branch_colors.contains_key(branch) {
                            let color = colors[color_index % colors.len()];
                            self.branch_colors.insert(branch.clone(), color);
                            color_index += 1;
                        }
                    }
                    commit.branches.first().and_then(|b| self.branch_colors.get(b)).copied().unwrap_or(Color::White)
                } else {
                    Color::White
                };

                self.commits.push(GraphNode {
                    commit,
                    column,
                    color,
                });
            }
        }

        pub fn render(&self, f: &mut Frame) {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.area());

            self.render_graph(f, chunks[0]);
            self.render_details(f, chunks[1]);
        }

        fn render_graph(&self, f: &mut Frame, area: Rect) {
            let mut lines = Vec::new();

            for (idx, node) in self.commits.iter().enumerate() {
                let is_selected = idx == self.selected_index;
                let prefix = if is_selected { "> " } else { "  " };

                let mut spans = vec![
                    Span::styled(prefix, Style::default().fg(Color::Yellow)),
                ];

                for col in 0..=node.column {
                    if col == node.column {
                        spans.push(Span::styled("●", Style::default().fg(node.color)));
                    } else {
                        spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));
                    }
                    if col < node.column {
                        spans.push(Span::styled("──", Style::default().fg(Color::DarkGray)));
                    }
                }

                spans.push(Span::styled(" ", Style::default()));
                spans.push(Span::styled(
                    &node.commit.hash[..7],
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ));
                spans.push(Span::styled(" ", Style::default()));

                for branch in &node.commit.branches {
                    spans.push(Span::styled(
                        format!("({})", branch),
                        Style::default()
                            .fg(self.branch_colors.get(branch).copied().unwrap_or(Color::White))
                            .add_modifier(Modifier::BOLD),
                    ));
                    spans.push(Span::styled(" ", Style::default()));
                }

                spans.push(Span::styled(
                    &node.commit.message,
                    Style::default().fg(if is_selected { Color::Yellow } else { Color::White }),
                ));

                lines.push(Line::from(spans));
            }

            let graph_text = Text::from(lines);
            let paragraph = Paragraph::new(graph_text)
                .block(Block::default().borders(Borders::ALL).title("Git History"))
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
        }

        fn render_details(&self, f: &mut Frame, area: Rect) {
            if let Some(node) = self.commits.get(self.selected_index) {
                let commit = &node.commit;
                let details = vec![
                    Line::from(vec![
                        Span::styled("Commit: ", Style::default().fg(Color::Cyan)),
                        Span::styled(&commit.hash, Style::default().add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Author: ", Style::default().fg(Color::Green)),
                        Span::styled(&commit.author, Style::default()),
                    ]),
                    Line::from(vec![
                        Span::styled("Date: ", Style::default().fg(Color::Green)),
                        Span::styled(&commit.date, Style::default()),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Message:", Style::default().fg(Color::Yellow)),
                    ]),
                    Line::from(commit.message.as_str()),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Branches:", Style::default().fg(Color::Magenta)),
                    ]),
                ];

                let mut branch_lines = Vec::new();
                for branch in &commit.branches {
                    branch_lines.push(Line::from(vec![
                        Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            branch,
                            Style::default()
                                .fg(self.branch_colors.get(branch).copied().unwrap_or(Color::White))
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]));
                }

                let mut all_lines = details;
                all_lines.extend(branch_lines);

                let paragraph = Paragraph::new(Text::from(all_lines))
                    .block(Block::default().borders(Borders::ALL).title("Commit Details"))
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Left);

                f.render_widget(paragraph, area);
            } else {
                let paragraph = Paragraph::new("No commit selected")
                    .block(Block::default().borders(Borders::ALL).title("Commit Details"))
                    .alignment(Alignment::Center);
                f.render_widget(paragraph, area);
            }
        }

        pub fn handle_key(&mut self, key: KeyCode) -> bool {
            match key {
                KeyCode::Up => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                    true
                }
                KeyCode::Down => {
                    if self.selected_index < self.commits.len().saturating_sub(1) {
                        self.selected_index += 1;
                    }
                    true
                }
                KeyCode::Char('q') => false,
                _ => true,
            }
        }
    }

    pub fn run_tui() {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();

        let mut graph = GitGraph::new();
        graph.load_history();

        let mut should_quit = false;

        while !should_quit {
            terminal
                .draw(|f| {
                    graph.render(f);
                })
                .unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                should_quit = !graph.handle_key(key.code);
            }
        }

        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen
        ).unwrap();
    }
}
