use bevy::prelude::*;
use crate::{
    components::*,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH}, utils::random_pipe_position,
};
pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ){

    commands.spawn(Camera2dBundle::default());  //2d 카메라 스폰

    commands.spawn((    //background sprite 스폰 ("C:\sakabam\assets\texture\background.png")
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            sprite: Sprite{
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., WINDOW_HEIGHT)),  //배경 파일이 288px, 그냥 *2 때리면 양옆으로 늘어나서 이렇게 하는거임
                ..default()
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true, // x축으로 sprite가 반복되게
            tile_y: false, // y축으로는 반복되지 않게
            stretch_value: 1., // stretch는 사이즈 조절, 1.이면 원본 사이즈
        },
        Background,
    ));

    commands.spawn((    //ground 스폰
        SpriteBundle{
            texture: asset_server.load("texture/base.png"),     //"C:\sakabam\assets\texture\base.png"
            sprite: Sprite{
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., 112.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -250., 1.),
            ..default()
        },
        ImageScaleMode::Tiled { 
            tile_x: true, 
            tile_y: false, 
            stretch_value: 1. 
        },
        Ground,
    ));

    commands.spawn((    //gameovertext 스폰
        SpriteBundle{
            texture: asset_server.load("texture/game-over.png"),     //("C:\sakabam\assets\texture\game-over.png")
            transform: Transform::from_xyz(0., 0., 1.),
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText(Timer::from_seconds(0.5, TimerMode::Once)),
    ));

    commands.spawn((    //pressspacebartext 스폰
        SpriteBundle{
            texture: asset_server.load("texture/space.png"),     //("C:\sakabam\assets\texture\space.png")
            transform: Transform::from_xyz(0., -50., 1.),
            ..default()
        },
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));


    //score용    
    let number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);   //얼만큼 크게 자를지 그 기준, 좌우 타일 갯수, 위아래 타일 갯수
    let number_texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(number_layout); //위에서 어떻게 자를지 정의후 그 변수로 여기서 메서드 불러서 실제로 자르는거임
    //그럼 바로 위 이 변수가 .add메서드가 되는거지

    for i in 0..3{
        let starting_point = -350. + (i as f32 * (24. + 2.));   //24가 스프라이트 크기라 24 + 2로...실질적 간격은 2인거임

        commands.spawn((    //scoretext 스폰
            SpriteBundle{
                texture: asset_server.load("texture/numbers.png"),     //("C:\sakabam\assets\texture\numbers.png")
                transform: Transform::from_xyz(starting_point, 200., 1.),
                ..default()
            },
            TextureAtlas{
                index: 0,   //이 인덱스는 잘려있는 여러개의 스프라이트 타일의 인덱스 넘버
                layout: number_texture_atlas_layout.clone(),    //.add메서드를 여기다가 바로 쓰지 않은 이유는 여러개를 써야해서 .clone때문에 그런건가? 몰?루
            },
            ScoreText,
        ));
    }

    commands.spawn((    //사카밤 스폰
        SpriteBundle {
            texture: asset_server.load("texture/fishsheet.png"),
            transform: Transform::from_xyz(0., 0., 2.),
            ..default()
        },
        TextureAtlas {
            index: 1,
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::new(100, 100),
                2,
                2,
                None,
                None,
            )),
        },
        Fish{
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            velocity: 0.,
        },
    ));
    
    for i in 0..5{
        let delta_x = i as f32 * 200.;  //파이프 간격
        let (lower_y, upper_y) = random_pipe_position();
        let mut pipe_transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);

        commands.spawn((    //Lowerpipe 스폰
            SpriteBundle{
                texture: asset_server.load("texture/pipe.png"),
                transform: pipe_transform,
                ..default()
            },
            LowerPipe,
        ));

        pipe_transform.rotate(Quat::from_rotation_z(std::f32::consts::PI)); //z 로테이션 돌리고
        pipe_transform.translation.y = upper_y;       //y좌표 조절하고

        commands.spawn((    //Upperpipe 스폰
            SpriteBundle {
                texture: asset_server.load("texture/pipe.png"),
                transform: pipe_transform,
                ..default()
            },
            UpperPipe{
                passed: false,
            },
        ));
    }
}