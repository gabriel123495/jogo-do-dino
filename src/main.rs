use macroquad::prelude::*;
use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Jogo do Dino".to_string(),
        fullscreen: false,
        window_width: 800,
        window_height: 450,
        high_dpi: false,
        ..Default::default()
    }
}

struct Cacto {
    x: f32,
    y: f32,
    base: f32,
    altura: f32,
}

impl Cacto {
    fn new(x: f32, y: f32, base: f32, altura: f32) -> Self {
        Self { x, y, base, altura }
    }

    fn desenhar(&self) {
        let p1 = vec2(self.x, self.y);
        let p2 = vec2(self.x + self.base / 2.0, self.y - self.altura);
        let p3 = vec2(self.x + self.base, self.y);

        draw_triangle(p1, p2, p3, GRAY);
    }

    fn mover(&mut self, velocidade: f32) {
        self.x -= velocidade;
        if self.x + self.base < 0.0 {
            self.x = screen_width() + rand::gen_range(0.0, 300.0);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let dino_texture = load_texture("assets/dino.png").await.unwrap();

    let dino_largura = 50.0;
    let dino_altura = 50.0;

    let mut dino_y = 350.0 - dino_altura;
    let dino_x = 50.0;
    let mut velocidade_y = 0.0;
    let gravidade = 0.6;
    let pulo = -12.0 * 1.3; 

    let mut cactos = vec![
        Cacto::new(800.0, 350.0, 40.0 * 0.7, 60.0 * 0.7),
        Cacto::new(1100.0, 350.0, 40.0 * 0.7, 60.0 * 0.7),
        Cacto::new(1400.0, 350.0, 40.0 * 0.7, 60.0 * 0.7),
    ];

    let mut pontuacao: u32 = 0;
    let mut velocidade_jogo: f32 = 5.0;

    loop {
        clear_background(WHITE);

        draw_rectangle(0.0, 350.0, screen_width(), 10.0, DARKGRAY);

        if is_key_pressed(KeyCode::Space) && dino_y >= 350.0 - dino_altura {
            velocidade_y = pulo;
        }
        velocidade_y += gravidade;
        dino_y += velocidade_y;

        if dino_y > 350.0 - dino_altura {
            dino_y = 350.0 - dino_altura;
            velocidade_y = 0.0;
        }

        draw_texture_ex(
            &dino_texture,
            dino_x,
            dino_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dino_largura, dino_altura)),
                ..Default::default()
            },
        );

        for cacto in &mut cactos {
            cacto.mover(velocidade_jogo);
            cacto.desenhar();

            let dino_rect = Rect::new(dino_x, dino_y, dino_largura, dino_altura);
            let cacto_rect = Rect::new(cacto.x, cacto.y - cacto.altura, cacto.base, cacto.altura);
            if dino_rect.overlaps(&cacto_rect) {
                draw_text(
                    &format!("Game Over! Pontuação: {}", pontuacao),
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    40.0,
                    RED,
                );
                next_frame().await;
                return;
            }
        }

        pontuacao += 1;

        if pontuacao % 500 == 0 {
            velocidade_jogo += 0.5;
        }

        draw_text(
            &format!("Pontuação: {}", pontuacao),
            10.0,
            30.0,
            30.0,
            BLACK,
        );

        next_frame().await;
    }
}
