use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GameOverText(pub Timer);

#[derive(Component)]
pub struct PressSpaceBarText(pub Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Fish{
    pub timer: Timer,
    pub velocity: f32,  //속도 근데 이제 낙하 속도 ㅇㅅㅇ    ㅎㅅㅎ     @ㅅ@      ㅡㅅㅡ     ㄴㅇㄱ
}

#[derive(Component)]
pub struct UpperPipe{
    pub passed: bool,
}

#[derive(Component)]
pub struct LowerPipe;