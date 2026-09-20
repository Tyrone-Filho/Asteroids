use macroquad::prelude::*;

struct Nave{
    posicao: Vec2,
    velocidade: Vec2,
    rotacao: f32,
}
struct Tiro{
    posicao: Vec2,
    velocidade: Vec2,
    rotacao: f32,
    vivo:bool
}
struct Asteroid{
    posicao:Vec2,
    velocidade:Vec2,
    rotacao:f32,
    velocidade_rotacao:f32,
    sides:u8,
    vivo:bool,
    raio:f32,
}

//fiquei cansado de criar asteroide
fn criar_asteroide(sides_param:u8,jogador_pos:Vec2,distancia_segura:f32)->Asteroid{

    let raio: f32 = rand::gen_range(10., 50.);
    let posicao = loop{
        let p = Vec2{ x:rand::gen_range(0., screen_width()), y: rand::gen_range(0., screen_height()) };
        if p.distance(jogador_pos) > distancia_segura + raio{
            break p;
        };
    };
    Asteroid {
        posicao: posicao, 
        velocidade: Vec2 { x: rand::gen_range(-0.5 , 0.5), y: rand::gen_range(-0.5 , 0.5) }, 
        rotacao:rand::gen_range(-0.2, 0.2),
        velocidade_rotacao:rand::gen_range(-0.5, 0.5),
        sides: sides_param,
        vivo:true,
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
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut gameover:bool = false;
    let mut jogador: Nave = Nave{
        posicao: Vec2::new(screen_width()/2.0, screen_height()/2.0),
        velocidade: Vec2::new(0.0, 0.0),
        rotacao: 0.0,
    };
    let mut tiros: Vec<Tiro> = Vec::new();
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
                tiros = Vec::new();
                asteroides = Vec::new();
                for _ in 0..rand::gen_range(4, 8){
                    asteroides.push(criar_asteroide(rand::gen_range(4, 7),jogador.posicao,DISTANCIA_SEGURA_CRIACAO));
                }
            }
            next_frame().await;
            continue;
        }
        clear_background(LIGHTGRAY);


        
        //Logica Asteroide
        for asteroid in asteroides.iter_mut(){
            if asteroid.posicao.x < 0.{
                asteroid.posicao.x = screen_width();
            }else if asteroid.posicao.x > screen_width(){
                asteroid.posicao.x = 0.;
            }
            if asteroid.posicao.y < 0.{
                asteroid.posicao.y = screen_height();
            }else if asteroid.posicao.y > screen_height(){
                asteroid.posicao.y = 0.;
            }
        }

        //Logica de tiro
        if is_key_pressed(KeyCode::Space) && tiros.len()<5{
            let tiro = Tiro{
                posicao: Vec2 { 
                    x: jogador.posicao.x + jogador.rotacao.sin() * ALTURA_NAVE,
                    y: jogador.posicao.y - jogador.rotacao.cos()*ALTURA_NAVE
                },
                velocidade: Vec2{
                    x: 5.0*jogador.rotacao.sin(),
                    y: -5.0*jogador.rotacao.cos(),
                },
                rotacao: jogador.rotacao,
                vivo:true
            };
            tiros.push(tiro);
        }

        //Logica Andar e Rotacionar

        if is_key_down(KeyCode::Up){
            jogador.velocidade.x += 0.2*jogador.rotacao.sin();
            jogador.velocidade.y -= 0.2*jogador.rotacao.cos();
        }

        if is_key_down(KeyCode::Right){
            jogador.rotacao += 0.025;
        }else if is_key_down(KeyCode::Left){
            jogador.rotacao -= 0.025;
        }
        // o codigo abaixo cria um limite de velocidade maximo em
        // maximo de aumento de velocidade * atrito /  1-atrito
        // entao ele estabiliza +/- em
        // 0.2 * 0.975 / 0.025
        //78/10 = 7.8 
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

        let vertice_1 = Vec2{
            x: jogador.posicao.x + jogador.rotacao.sin() * ALTURA_NAVE,
            y: jogador.posicao.y - jogador.rotacao.cos()*ALTURA_NAVE
        };
        let vertice_2 = Vec2{
            x: jogador.posicao.x - LARGURA_NAVE*jogador.rotacao.cos()/3. - jogador.rotacao.sin()*ALTURA_NAVE/3.,
            y: jogador.posicao.y + ALTURA_NAVE*jogador.rotacao.cos()/3. - ALTURA_NAVE*jogador.rotacao.sin()/3.,
        };
        let vertice_3 = Vec2{
            x: jogador.posicao.x + LARGURA_NAVE*jogador.rotacao.cos()/3. - jogador.rotacao.sin()*ALTURA_NAVE/3.,
            y: jogador.posicao.y + ALTURA_NAVE*jogador.rotacao.cos()/3. + ALTURA_NAVE*jogador.rotacao.sin()/3.,
        };

        draw_triangle_lines( vertice_1, vertice_2, vertice_3, 1.0, BLACK);

        //Logica tiro acertar Asteroid / player morrer
        for asteroide in asteroides.iter_mut(){
            for tiro in tiros.iter_mut(){
                if asteroide.posicao.distance_squared(tiro.posicao) < asteroide.raio.powi(2) {
                    tiro.vivo = false;
                    asteroide.vivo = false;
                }
            }
            if asteroide.posicao.distance_squared(jogador.posicao) < (asteroide.raio+ ALTURA_NAVE/3.).powi(2){
                gameover = true;
            }
        }
        //Logica asteroide se duplicar
        let mut novos_asteroides: Vec<Asteroid> = Vec::new();
        for asteroide in asteroides.iter_mut(){
            if !asteroide.vivo && asteroide.sides > 4{
                for _ in 0..(asteroide.sides-2){
                    let mut novo_asteroide = criar_asteroide(asteroide.sides - 1,jogador.posicao,10.);   
                    novo_asteroide.posicao = asteroide.posicao + Vec2{x: rand::gen_range(-50., 50.),y: rand::gen_range(-50., 50.)};
                    novos_asteroides.push(novo_asteroide);
                }
            }
        }
        asteroides.retain(|asteroid| asteroid.vivo);
        asteroides.extend(novos_asteroides);
        if asteroides.len()<=0{
            gameover = true;
            continue;
        }

        //Logica desenhar Asteroide
        for asteroid in asteroides.iter_mut(){
            draw_poly_lines(asteroid.posicao.x, asteroid.posicao.y, asteroid.sides,
                asteroid.raio, asteroid.rotacao, 1., BLACK);
            asteroid.rotacao += asteroid.velocidade_rotacao;
            asteroid.posicao += asteroid.velocidade
        }


        //Logica desenhar tiro
        for tiro in tiros.iter_mut(){
            draw_rectangle_ex(
                tiro.posicao.x, 
                tiro.posicao.y, 
                2., 
                8., 
                DrawRectangleParams {
                    offset: vec2(0., 0.), // ponto de pivô
                    rotation: tiro.rotacao, // em radianos
                    color: BLACK,
                });
            tiro.posicao += tiro.velocidade;
            if (tiro.posicao.x<0. || tiro.posicao.x>screen_width()) || (tiro.posicao.y<0. || tiro.posicao.y>screen_height()){
                // tira todo tiro : &Tiro que esteja fora da tela
                // em outras palavras, matem apenas os que estao na tela
                tiro.vivo = false;
            }
        }
        tiros.retain(|tiro| tiro.vivo);
        next_frame().await
    }
}