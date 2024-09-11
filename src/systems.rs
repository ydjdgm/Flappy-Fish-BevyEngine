use std::time::Duration;

use bevy::{ecs::query, prelude::*};

use crate::{components::*, constants::WINDOW_WIDTH, utils::random_pipe_position, Game, GameState};

pub fn blink_space_bar_text(
    time: Res<Time>,
    mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
){
    let (mut space, mut visibility) = query.single_mut();

    let timer = &mut space.0;
    timer.tick(time.delta());

    if timer.finished(){
        if *visibility == Visibility::Hidden{
            *visibility = Visibility::Visible;
        }else{
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn move_background(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Background>>
){
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;  //움직이는 속도

    background_transform.translation.x -= delta_x;
    if background_transform.translation.x < -288.{      //배경 무한 지속
        background_transform.translation.x = 0.;
    }
}

pub fn move_ground(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ground>>,
){
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta; //움직이는 속도

    ground_transform.translation.x -= delta_x;

    if ground_transform.translation.x < -288.{
        ground_transform.translation.x = 0.;
    }
}

pub fn animate_fish(    //사카밤 애니메이션
    time: Res<Time>,
    mut query: Query<(&mut Fish, &mut TextureAtlas)>
){
    for (mut fish, mut texture_atlas) in query.iter_mut(){
        let delta = time.delta();

        fish.timer.tick(delta);

        if fish.timer.finished(){
            texture_atlas.index = if texture_atlas.index == 2{  //인덱스가 2면 다시 0으로
                0
            }else{
                texture_atlas.index + 1     //2아니면 +1
            };
        }
    }
}

pub fn start_game(  
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut space_query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
    mut text_query: Query<&mut GameOverText>,
    mut fish_query: Query<(&mut Fish, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Fish>)>,
    mut lower_pipe_query: Query<
    &mut Transform,
    (With<LowerPipe>, Without<Fish>, Without<UpperPipe>),
>,
){
    if !keyboard_input.just_pressed(KeyCode::Space){    //스페이스바 눌리면
        return;
    }

    if game.state == GameState::GameOver {

        game.score = 0;
        for (i, (mut transform, mut upper_pipe)) in upper_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;
 
            upper_pipe.passed = false;
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
 
        for (i, mut transform) in lower_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;
 
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
    };

    game.state = GameState::Active;     //gamestate = active

    for (mut fish, mut transform) in fish_query.iter_mut(){ //게임 시작 시 사카밤 위치 초기화
        fish.velocity = 0.;
        transform.translation.y = 0.;
        transform.rotation = Quat::from_rotation_z(0.);
    }

    let (mut space, mut visibility) = space_query.single_mut();//inactive and gameover에서 보이는 텍스트 visibility = Hidden
    space.0.reset();
    *visibility = Visibility::Hidden;

    let mut game_over_visibility = game_over_query.single_mut();
    *game_over_visibility = Visibility::Hidden;
}

pub fn gravity(     //사카밤 중력
    time: Res<Time>, 
    mut game: ResMut<Game>,
    mut query: Query<(&mut Fish, &mut Transform)>,
    mut text_query: Query<&mut GameOverText>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){     
    for (mut fish, mut transfrom) in query.iter_mut(){      //막간 러스트 메서드 공부 시간ㅎㅋㅋ허허허   iter.mut란? 해당 컬렉션의 가변 참조를 반복할 수 있게 함 <- 이게 무슨 소리냐고? 나도 몰라 시발...그니까 요약 하자면 변수 소유권을 유지한채로 가변 가능한 참조를 하는거임
        let delta = time.delta().as_secs_f32();     //시간이 지날수록 delta가 쌓이고
        let gravity = 9.8;
        let delta_v = gravity * 150. * delta;   //delta velocity //여기서 delta가 곱해져서 시간 지날수록 난이도 업 //이이이이이이이이이이잉이이이이이이이이이이이이이이이이이이이이이이이잉이이이이이이이이이이이이이이이이이이이이이이잉이이이이이이이이이이이이이이이이이이이이이이이이이이이이ㅣ이이이이이잉
        let delta_y = fish.velocity * delta;
        let new_y = (transfrom.translation.y + delta_y).min(260.);  //.min으로 (transfrom.translation.y + delta_y) 랑 260.을 비교해서 260을 못넘게 해버리는거임 뿌슝빠슝뿌슝 상상도 못한 메서드 끼야아아악

        transfrom.translation.y = new_y;
        fish.velocity -= delta_v;
        transfrom.translation.y += fish.velocity * delta;

        //회전
        let rotation = fish.velocity / 600.;//속도 따라서 각 변화
        let max_rotation = 0.5; //최대 각 설정
        transfrom.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));//사카사카밤밤바스피스피스 이이이이이이잉ㅇ 기모링ㅇㅇㅇ

        let ground_y = -250.;
        let ground_height = 112.;
        let fish_height = 24.;  //보면서 조정 필요

        let collision_point = ground_y + ground_height / 2. + fish_height / 2.; //(ground_height / 2) = ground의 중앙, (fish_height / 2) = fish의 중앙 <- 이 둘을 더하고 ground_y(ground.trans.y값)을 더하면 얘는 음수니까(-250) 결과적으로 colPnt가 나오는 거임

        if transfrom.translation.y < collision_point{   //땅에 닿으면
            transfrom.translation.y = collision_point;  //위치 고정
            fish.velocity = 0.;                         //속도 = 0

            game.state = GameState::GameOver;           //게임오버
            *game_over_query.single_mut() = Visibility::Visible;    //텍스트 visible


            //사운드
            commands.spawn(AudioBundle{
                source: asset_server.load("audio/hit.ogg"),  //(assets\audio\hit.ogg)
                settings: PlaybackSettings::DESPAWN,//끝나면 자동 디스폰
                ..default()
            });
        }
    }
}//tlqkfㅋㅋㅋㅋ 웰케 하찮게 떨어지냐ㅋㅋ 개웃기네ㅋㅋㅋㅋㅋ

//여긴 어디 나는 누구
//여긴 페나코니 나는 반디 헤헤

pub fn jump(
    mut query: Query<&mut Fish>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if !keyboard_input.just_pressed(KeyCode::Space){
        return;
    }
    commands.spawn(AudioBundle{ //효과음 스폰
        source: asset_server.load("audio/wing.ogg"),  //(assets\audio\wing.ogg)
        settings: PlaybackSettings::DESPAWN,    //자동 디스폰
        ..default()
    });
    for mut fish in query.iter_mut(){
        fish.velocity = 300.;
    }
}

pub fn pipes(
    time: Res<Time>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &mut Transform)>,
    mut lower_pipe_query: Query<(&LowerPipe, &mut Transform), Without<UpperPipe>>,
    mut fish_query: Query<&Transform, (With<Fish>, Without<LowerPipe>, Without<UpperPipe>)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut commands: Commands,
){
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta;

    let utmost_right_pipe = upper_pipe_query
    .iter() //가변으로 해놓고
    .max_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap())    //x값 비교
    .unwrap()
    .1
    .translation
    .x;

    let new_pipe_position = utmost_right_pipe + 200.;
    let (lower_y, upper_y) = random_pipe_position();
    let out_of_screen_x = (-WINDOW_WIDTH / 2.) - 26.;

    for (mut upper_pipe, mut transform) in upper_pipe_query.iter_mut(){  //trans.x를 -해서 왼쪽으로 이동하게
        transform.translation.x -= delta_x;
        
        if transform.translation.x < out_of_screen_x{
            transform.translation.x = new_pipe_position;
            transform.translation.y = upper_y;
            upper_pipe.passed = false;
        }
    }



    for (_, mut transform) in lower_pipe_query.iter_mut(){
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = lower_y;
        }
    }

    let is_collision = |fish_transform: &Transform, pipe_transform: &Transform| -> bool {
        let fish_x = fish_transform.translation.x;
        let fish_y = fish_transform.translation.y;
        let fish_width = 34.0;
        let fish_height = 24.0;
     
        let pipe_x = pipe_transform.translation.x;
        let pipe_y = pipe_transform.translation.y;
        let pipe_width = 52.0;
        let pipe_height = 320.0;
     
        let collision_x = fish_x + fish_width / 2.0 > pipe_x - pipe_width / 2.0
            && fish_x - fish_width / 2.0 < pipe_x + pipe_width / 2.0;
        let collision_y = fish_y + fish_height / 2.0 > pipe_y - pipe_height / 2.0
            && fish_y - fish_height / 2.0 < pipe_y + pipe_height / 2.0;
     
        collision_x && collision_y
    };

    for fish_transform in fish_query.iter_mut() {
        let mut game_over = || {
            game.state = GameState::GameOver;
            *game_over_query.single_mut() = Visibility::Visible;
 
            //game over 사운드
            commands.spawn(AudioBundle {
                source: asset_server.load("audio/hit.ogg"), //(assets\audio\hit.ogg)
                settings: PlaybackSettings::DESPAWN,
            });
        };
 
        for (_, transform) in upper_pipe_query.iter_mut() {
            if is_collision(fish_transform, &transform) {
                game_over();
            }
        }
 
        for (_, transform) in lower_pipe_query.iter_mut() {
            if is_collision(fish_transform, &transform) {
                game_over();
            }
        }
    }
}

// systems.rs
 
pub fn score(
    mut game: ResMut<Game>,
    fish_query: Query<(&Fish, &Transform)>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (_, fish_transform) in fish_query.iter() {
        for (mut upper_pipe, transform) in upper_pipe_query.iter_mut() {
            let passed = transform.translation.x < fish_transform.translation.x;
            let passed_state = upper_pipe.passed;
 
            if passed && !passed_state {
                game.score += 1;
                upper_pipe.passed = true;
 
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/point.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                });
 
                println!("Score: {}", game.score);
            }
        }
    }
}

pub fn render_score(game: Res<Game>, mut query: Query<&mut TextureAtlas, With<ScoreText>>) {
    let score_string = format!("{:03}", game.score);
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
 
    for (digit, mut texture_atlas) in score_digits.iter().zip(query.iter_mut()) {
        texture_atlas.index = *digit;
    }
}