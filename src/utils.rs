use rand::Rng;

pub fn random_pipe_position() -> (f32, f32){        //y값을 랜덤하게 하기 위해 lower의 y값 뽑고, +450 해서 upper꺼 까지 둘 다 리턴
    let mut rng = rand::thread_rng();
    let lower = -rng.gen_range(70.0..200.0);

    (lower, lower + 450.0)
}