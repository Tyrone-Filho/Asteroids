#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//Comando para nao abrir o terminal no windows
use macroquad::prelude::*;

struct Nave{
    posicao: Vec2,
    velocidade: Vec2,
    rotacao: f32,
}
struct Laser{
    largura:f32,
    dano:f32,
    laser_scale:f32,
    acertando:bool
}
struct Asteroid{
    posicao:Vec2,
    velocidade:Vec2,
    rotacao:f32,
    velocidade_rotacao:f32,
    sides:u8,
    hp:f32,
    raio:f32,
}

//fiquei cansado de criar asteroide
fn criar_asteroide(sides_param:u8,jogador_pos:Vec2,distancia_segura:f32)->Asteroid{

    let raio: f32 = rand::gen_range(10., 50.);
    let posicao = loop{
        let p = Vec2{ x:rand::gen_range(0., screen_width()), y: rand::gen_range(0., screen_height()) };
        if p.distance_squared(jogador_pos) > (distancia_segura + raio).powi(2){
            break p;
        };
    };
    Asteroid {
        posicao: posicao, 
        velocidade: Vec2 { x: rand::gen_range(-0.5 , 0.5), y: rand::gen_range(-0.5 , 0.5) }, 
        rotacao:rand::gen_range(-0.2, 0.2),
        velocidade_rotacao:rand::gen_range(-0.5, 0.5),
        sides: sides_param,
        hp:(20*sides_param) as f32,
        raio: raio
    }
}

const DISTANCIA_SEGURA_CRIACAO:f32 = 150.0;
const ALTURA_NAVE: f32 = 30.0;
const LARGURA_NAVE: f32 = 37.0;
fn conf() -> Conf {
    //unica parte que "nao fiz" pois n consigui entender direito e queria setar em 60fps
    Conf {
        window_title: "Asteroids".to_owned(),
        platform: miniquad::conf::Platform{
            swap_interval: Some(1),
            ..Default::default()
        },
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
for _ in 0..30{
    next_frame().await;
}
let mut gameover:bool = false;
let mut jogador: Nave = Nave{
    posicao: Vec2::new(screen_width()/2.0, screen_height()/2.0),
    velocidade: Vec2::new(0.0, 0.0),
    rotacao: 0.0,
};
let mut boost:f32;

//LASER
let hitbox_laser:f32 = LARGURA_NAVE*2.;
let laser_base = Laser{
    largura:1.0,dano:0.7, laser_scale:1.0,acertando:false
};
let laser_quantidade_max:usize = 3;
let mut laser_disponiveis:Vec<Laser> = Vec::new();
for _ in 0..laser_quantidade_max{
    laser_disponiveis.push(Laser { 
            largura: laser_base.largura, 
            dano: laser_base.dano, 
            laser_scale: laser_base.laser_scale,
            acertando: laser_base.acertando
        }
    );
}
//

let mut asteroides:Vec<Asteroid> = Vec::new();
macroquad::rand::srand(macroquad::time::get_time() as u64);

//Logica Asteroide
for _ in 0..rand::gen_range(4, 8){
    asteroides.push(criar_asteroide(rand::gen_range(4, 7),jogador.posicao,DISTANCIA_SEGURA_CRIACAO));
}


loop {
    if gameover{
        clear_background(LIGHTGRAY);
        let mut texto = "Você ganhou aperte [enter] para jogar novamente";
        let tamanho_fonte:f32 = 30.;
        if asteroides.len()>0{
            texto = "Você perdeu aperte [enter] para jogar novamente";
        }
        let tamanho_texto = measure_text(texto, None, tamanho_fonte as u16, 1.0);
        
        draw_text(texto, screen_width()/2. - tamanho_texto.width/2., screen_height()/2. - tamanho_texto.height/2., tamanho_fonte, BLACK);
        if is_key_down(KeyCode::Enter){
            gameover = false;
            jogador = Nave{
                posicao: Vec2::new(screen_width()/2.0, screen_height()/2.0),
                velocidade: Vec2::new(0.0, 0.0),
                rotacao: 0.0,
            };
            asteroides = Vec::new();
            for _ in 0..rand::gen_range(4, 8){
                asteroides.push(criar_asteroide(rand::gen_range(4, 7),jogador.posicao,DISTANCIA_SEGURA_CRIACAO));
            }
        }
        next_frame().await;
        continue;
    }
    clear_background(LIGHTGRAY);


    
    
    
    //LOGICA TECLAS EM GERAL
    if is_key_pressed(KeyCode::Delete){
        let mut boss = criar_asteroide(4, jogador.posicao, 10.);
        boss.velocidade = Vec2::ZERO;
        boss.posicao = Vec2 { x: 200.+(20*asteroides.len()) as f32, y: 200. };
        boss.hp = 50000.;
        boss.raio = 80.;
        boss.velocidade_rotacao = 0.;
        asteroides.push(boss);
    }
    //Logica Andar e Rotacionar 
    if is_key_pressed(KeyCode::Q){
        for aste in asteroides.iter_mut(){
            aste.velocidade = Vec2::ZERO;
        }
    }
    if is_key_down(KeyCode::LeftShift){
        boost = 0.2;

    }else{
        boost = 0.;
    }
    if is_key_down(KeyCode::Up){
        jogador.velocidade.x += (0.1+boost)*jogador.rotacao.sin();
        jogador.velocidade.y -= (0.1+boost)*jogador.rotacao.cos();
    }

    if is_key_down(KeyCode::Right){
        jogador.rotacao += 0.025;
    }else if is_key_down(KeyCode::Left){
        jogador.rotacao -= 0.025;
    }

    //LOGICA ACELERAR
    // o codigo abaixo cria um limite de velocidade maximo em
    // maximo de aumento de velocidade * atrito /  1-atrito
    jogador.posicao += jogador.velocidade;
    jogador.velocidade *= 0.975;

    if jogador.posicao.x > screen_width(){
        jogador.posicao.x = 0.0;
    }else if jogador.posicao.x < 0.0{
        jogador.posicao.x = screen_width();
    }
    if jogador.posicao.y > screen_height(){
        jogador.posicao.y = 0.0;
    }else if jogador.posicao.y < 0.0{
        jogador.posicao.y = screen_height();
    }


    //Reseta os lasers
    for laser in laser_disponiveis.iter_mut() {
        laser.acertando = false;
    }


    //Logica laser acertar Asteroid / player morrer
    let mut lasers_usados:usize = 0;
    let mut novos_asteroides: Vec<Asteroid> = Vec::new();

    //Logica Asteroide
        

    for asteroide in asteroides.iter_mut(){
        //LOOPAR PELA TELA
        if asteroide.posicao.x < 0.{
            asteroide.posicao.x = screen_width();
        }else if asteroide.posicao.x > screen_width(){
            asteroide.posicao.x = 0.;
        }
        if asteroide.posicao.y < 0.{
            asteroide.posicao.y = screen_height();
        }else if asteroide.posicao.y > screen_height(){
            asteroide.posicao.y = 0.;
        }

        //COLISAO COM LASER
        if asteroide.posicao.distance_squared(jogador.posicao) < (asteroide.raio+hitbox_laser).powi(2) {
            if lasers_usados < laser_quantidade_max{
                let laser_atual: &mut Laser = &mut laser_disponiveis[lasers_usados];
                laser_atual.acertando = true;
                asteroide.hp -= laser_atual.dano;
                laser_atual.dano += 0.1*get_frame_time()*laser_atual.laser_scale;
                
                //LOGICA DESENHAR LASER
                if laser_atual.largura>8.{
                    laser_atual.largura+= 1.5*get_frame_time();
                }

                let origem = jogador.posicao;
                let meio_asteroid = asteroide.posicao;
                let v = meio_asteroid-origem;
                let direcao = v.normalize();


                if laser_atual.largura>7.{
                    draw_line(jogador.posicao.x, jogador.posicao.y, asteroide.posicao.x, asteroide.posicao.y, laser_atual.largura, RED);
                    draw_line(jogador.posicao.x, jogador.posicao.y, asteroide.posicao.x, asteroide.posicao.y, 7., Color { r: 0.8, g: 0.16, b: 0.22, a: 1. });
                    draw_line(jogador.posicao.x, jogador.posicao.y, asteroide.posicao.x, asteroide.posicao.y, 4., Color { r: 0.7, g: 0.16, b: 0.22, a: 1. });
                }
                else if laser_atual.largura>4.{
                    draw_line(jogador.posicao.x, jogador.posicao.y, asteroide.posicao.x, asteroide.posicao.y, laser_atual.largura, RED);
                    draw_line(jogador.posicao.x, jogador.posicao.y, asteroide.posicao.x, asteroide.posicao.y, 1., Color { r: 0.8, g: 0.16, b: 0.22, a: 1. });
                }
                else if laser_atual.largura>=1.{
                    draw_line(
                        (origem+hitbox_laser*direcao).x,
                        (origem+hitbox_laser*direcao).y,
                        asteroide.posicao.x, asteroide.posicao.y, laser_atual.largura, RED);
                }
                
            }

            //MORTE ASTEROIDE

            if asteroide.hp<=0. && asteroide.sides > 4{
                laser_disponiveis[lasers_usados] = Laser { 
                    largura: laser_base.largura, 
                    dano: laser_base.dano, 
                    laser_scale: laser_base.laser_scale,
                    acertando: laser_base.acertando
                };

                for _ in 0..(asteroide.sides-2){
                    let mut novo_asteroide = criar_asteroide(asteroide.sides - 1,jogador.posicao,hitbox_laser);   
                    novo_asteroide.posicao = asteroide.posicao + Vec2{x: rand::gen_range(-50., 50.),y: rand::gen_range(-50., 50.)};
                    while novo_asteroide.posicao.distance_squared(jogador.posicao) < (hitbox_laser+LARGURA_NAVE).powi(2){
                        novo_asteroide.posicao += Vec2{x: rand::gen_range(-10., 10.),y: rand::gen_range(-10., 10.)};
                    }
                    //adicionar pra ele ter mais hp dependendo do nivel
                    novos_asteroides.push(novo_asteroide);
                }
            }
            lasers_usados += 1;
        }

        //MORTE PLAYER

        if asteroide.posicao.distance_squared(jogador.posicao) < asteroide.raio.powi(2){
            gameover = true;
        }
    }
    //Ver quais lasers estao vivos
    for laser in laser_disponiveis.iter_mut(){
        if !laser.acertando{
            laser.dano = laser_base.dano;
            laser.largura = laser_base.largura;
        }
    }
    
    asteroides.retain(|asteroid| asteroid.hp>0.);
    asteroides.extend(novos_asteroides);
    if asteroides.len()<=0{
        gameover = true;
        continue;
    }

    //Logicas DESENHAR

    //Desenhar asteroide
    for asteroid in asteroides.iter_mut(){
        draw_poly_lines(asteroid.posicao.x, asteroid.posicao.y, asteroid.sides,
            asteroid.raio, asteroid.rotacao, 1., BLACK);
        asteroid.rotacao += asteroid.velocidade_rotacao;
        asteroid.posicao += asteroid.velocidade
    }

    //DESENHAR PLAYER
    let vertice_meio = Vec2{
    //Topo
        x: jogador.posicao.x + jogador.rotacao.sin()*LARGURA_NAVE/2.,
        y: jogador.posicao.y - jogador.rotacao.cos()*ALTURA_NAVE/2.
    };

    let vertice_1 = Vec2{
        //Meio
        x: jogador.posicao.x - jogador.rotacao.sin() * ALTURA_NAVE/2.,
        y: jogador.posicao.y + jogador.rotacao.cos() * ALTURA_NAVE/2.
    };
    let vertice_2 = Vec2{
        //ESQUERDA
        x: jogador.posicao.x +  -1.*jogador.rotacao.cos() * LARGURA_NAVE/2. - jogador.rotacao.sin()*ALTURA_NAVE,
        y: jogador.posicao.y + -1.*jogador.rotacao.sin() * LARGURA_NAVE/2. + jogador.rotacao.cos()*ALTURA_NAVE,
    };
    let vertice_3 = Vec2{
        //DIREITA
        x: jogador.posicao.x + jogador.rotacao.cos() * LARGURA_NAVE/2. - jogador.rotacao.sin()*ALTURA_NAVE,
        y: jogador.posicao.y + jogador.rotacao.sin() * LARGURA_NAVE/2. + jogador.rotacao.cos()*ALTURA_NAVE,
    };


    //draw_triangle_lines( vertice_1, vertice_2, vertice_3, 1.0, BLACK);
    draw_line(vertice_1.x, vertice_1.y, vertice_2.x, vertice_2.y, 1.0, BLACK);
    draw_line(vertice_1.x, vertice_1.y, vertice_3.x, vertice_3.y, 1.0, BLACK);
    draw_line(vertice_meio.x, vertice_meio.y, vertice_2.x, vertice_2.y, 1.0, BLACK);
    draw_line(vertice_meio.x, vertice_meio.y, vertice_3.x, vertice_3.y, 1.0, BLACK);
    draw_circle_lines(
        jogador.posicao.x, 
        jogador.posicao.y, 
        hitbox_laser, 1.,
            BLUE);
    if is_key_down(KeyCode::LeftShift){
        let variacao = rand::gen_range(1., 1.5);
        draw_triangle(
            Vec2{
                //ESQUERDA
                x: jogador.posicao.x +  -1.*jogador.rotacao.cos() * LARGURA_NAVE/4. - jogador.rotacao.sin()*ALTURA_NAVE*(1.0+boost)*variacao,
                y: jogador.posicao.y + -1.*jogador.rotacao.sin() * LARGURA_NAVE/4. + jogador.rotacao.cos()*ALTURA_NAVE*(1.0+boost)*variacao,
            },
            Vec2{
                //DIREITA
                x: jogador.posicao.x + jogador.rotacao.cos() * LARGURA_NAVE/4. - jogador.rotacao.sin()*ALTURA_NAVE*(1.0+boost)*variacao,
                y: jogador.posicao.y + jogador.rotacao.sin() * LARGURA_NAVE/4. + jogador.rotacao.cos()*ALTURA_NAVE*(1.0+boost)*variacao,
            },
        vertice_1,
            ORANGE);
    }

    next_frame().await
}
}

// quero me matar
// ficou uma merda
// vou tentar fazer 1 melhor no futuro, ou melhorar esse
// falta upgrades,niveis e etc
// mas tambem tenho que pegar leve so fazer 2 dias
// sou horrivel nesse tipo de matematica
