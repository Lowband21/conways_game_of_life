use druid::widget::prelude::*;
use druid::Color;
use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label},
    AppLauncher, Command, Lens, PlatformError, TimerToken, Widget, WindowDesc,
};

const WIDTH: usize = 30;
const HEIGHT: usize = 30;
const TIMER_ID: TimerToken = TimerToken::from_raw(0);
const TICK_DURATION: std::time::Duration = std::time::Duration::from_millis(500);

#[derive(Clone, Lens, Data)]
struct World {
    cells: [[bool; WIDTH]; HEIGHT],
    initial_cells: [[bool; WIDTH]; HEIGHT],
    current_cell: (usize, usize),
}

impl World {
    fn new() -> World {
        World {
            cells: [[false; WIDTH]; HEIGHT],
            initial_cells: [[false; WIDTH]; HEIGHT],
            current_cell: (0, 0),
        }
    }

    fn generate(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if rand::random() {
                    self.initial_cells[i][j] = true;
                }
            }
        }
    }

    fn live_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let ni = (HEIGHT as isize + x as isize + i) as usize % HEIGHT;
                let nj = (WIDTH as isize + y as isize + j) as usize % WIDTH;
                if self.cells[ni][nj] {
                    count += 1;
                }
            }
        }
        count
    }

    fn step(&self) -> World {
        let mut new_cells = self.cells.clone();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let live_neighbours = self.live_neighbours(i, j);
                new_cells[i][j] = match (self.cells[i][j], live_neighbours) {
                    (true, x) if x < 2 || x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };
            }
        }
        World {
            cells: new_cells,
            initial_cells: self.initial_cells.clone(),
            current_cell: self.current_cell,
        }
    }
}

#[derive(Clone, Lens, Data)]
struct AppState {
    world: World,
}

struct WorldWidget {}

impl Widget<AppState> for WorldWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_timer(TICK_DURATION);
            }
            Event::Timer(_) => {
                let (i, j) = data.world.current_cell;
                if i < HEIGHT && j < WIDTH {
                    let mut new_cells = data.world.cells.clone();
                    new_cells[i][j] = data.world.initial_cells[i][j];

                    let mut new_world = data.world.clone();
                    new_world.cells = new_cells;

                    if j + 1 < WIDTH {
                        new_world.current_cell.1 += 1;
                    } else if i + 1 < HEIGHT {
                        new_world.current_cell = (i + 1, 0);
                    }

                    data.world = new_world;

                    ctx.request_paint();
                    ctx.request_timer(std::time::Duration::from_millis(100));
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let size = ctx.size();
        let cell_width = size.width / WIDTH as f64;
        let cell_height = size.height / HEIGHT as f64;

        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let rect = druid::Rect::from_origin_size(
                    (j as f64 * cell_width, i as f64 * cell_height),
                    (cell_width, cell_height),
                );
                let color = if data.world.cells[i][j] {
                    Color::BLUE
                } else {
                    Color::WHITE
                };
                ctx.fill(rect, &color);
                ctx.stroke(rect, &Color::rgb8(200, 200, 200), 1.0); // Add stroke around each cell
            }
        }
    }
}

pub fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder())
        .title("Conway's Game of Life")
        .window_size((800.0, 800.0));

    let mut initial_world = World::new();
    initial_world.generate();
    initial_world.cells = initial_world.initial_cells.clone();

    let initial_state = AppState {
        world: initial_world,
    };

    AppLauncher::with_window(main_window).launch(initial_state)
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("Conway's Game of Life").with_text_size(24.0))
        .with_child(
            Button::new("Step").on_click(|_ctx, data: &mut AppState, _env| {
                data.world = data.world.step();
            }),
        )
        .with_child(WorldWidget {})
}
