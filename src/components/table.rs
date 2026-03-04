use ratatui::{
    layout::{Rect, Constraint},
    style::Style,
    widgets::{Table as RatatuiTable, Row, Cell, Block, Borders},
    Frame,
};
use crate::{Component, Context, ClickableArea};

pub struct Table<A> {
    pub rows: Vec<TableRow<A>>,
    pub header: Vec<String>,
    pub widths: Vec<Constraint>,
    pub title: Option<String>,
}

pub struct TableRow<A> {
    pub cells: Vec<String>,
    pub style: Option<Style>,
    pub action: Option<A>,
}

impl<A: Clone> Table<A> {
    pub fn new(header: Vec<String>, widths: Vec<Constraint>) -> Self {
        Self {
            rows: Vec::new(),
            header,
            widths,
            title: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(format!(" {} ", title.into()));
        self
    }

    pub fn add_row(mut self, cells: Vec<String>, style: Option<Style>, action: Option<A>) -> Self {
        self.rows.push(TableRow { cells, style, action });
        self
    }
}

impl<S, A: Clone> Component<S, A> for Table<A> {
    fn render(&self, f: &mut Frame, _state: &mut S, area: Rect, context: &mut Context<A>) {
        let mut block = Block::default().borders(Borders::ALL);
        if let Some(t) = &self.title {
            block = block.title(t.clone());
        }
        block = block.border_style(Style::default().fg(context.theme.secondary));

        let header_cells = self.header.iter().map(|h| Cell::from(h.as_str()).style(Style::default().fg(context.theme.primary)));
        let header = Row::new(header_cells).style(Style::default().add_modifier(ratatui::style::Modifier::BOLD));

        let rows: Vec<Row> = self.rows.iter().map(|r| {
            let cells = r.cells.iter().map(|c| Cell::from(c.as_str()));
            let mut row = Row::new(cells);
            if let Some(s) = r.style {
                row = row.style(s);
            } else {
                row = row.style(Style::default().fg(context.theme.text));
            }
            row
        }).collect();

        let table = RatatuiTable::new(rows, &self.widths)
            .header(header)
            .block(block);

        f.render_widget(table, area);

        // Register clickable areas for rows
        // Note: This is an approximation as Ratatui Table doesn't expose row offsets easily.
        // For simple tables without wrapping, each row is 1 line + 1 for header.
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(action) = &row.action {
                let row_area = Rect {
                    x: area.x + 1,
                    y: area.y + 2 + i as u16, // +1 border, +1 header
                    width: area.width.saturating_sub(2),
                    height: 1,
                };
                if row_area.y < area.y + area.height - 1 {
                    context.clickable_areas.push(ClickableArea {
                        area: row_area,
                        action: action.clone(),
                    });
                }
            }
        }
    }
}
