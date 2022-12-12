#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

mod game;
mod logger;
mod window;
mod settings;
mod context;
mod utils;
mod camera;
mod assets;
mod shaders;
mod cursor;
mod light;
mod ui;

fn main() {
    let game = game::Game::new();
    {
        let c = &game.context;
        c.load_assets("./assets/compiled.bin");
        c.add_object(
            c.assets.get_mesh("terrain_01"),
            shaders::terrain::Material::new(c.assets.get_texture("terrain_01")), 0
        );
        let mutant = c.add_object(
            c.assets.get_mesh("ch"),
            shaders::basic_anim::Material::new(c.assets.get_texture("ch_diffuse")), 1
        );
        mutant.instances.add(assets::InstanceTransform { position: [0.;3], scale: [0.01,0.01,0.01] });
        mutant.set_animation(c.assets.get_animation("ch_idle"));
        *c.character.lock().unwrap() = Some(mutant.clone());
        c.camera.lock().unwrap().set_target(camera::CameraTarget::Joint {
            object: mutant.clone(),
            offset: [0.,0.5,0.].into(),
            scale: 0.01,
            joint_id: 0
        });
        c.add_square(0.3, -0.3, 0.25, 0.25, [[1.0,1.,1.,0.5];4], ui::UIElementTexture::SunDepthBuffer);
    }
    game.start()
}