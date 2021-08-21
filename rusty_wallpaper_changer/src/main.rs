#![windows_subsystem = "windows"]
//la linea de arriba es para no generar ventana

extern crate rand;

// para el random
use rand::thread_rng;
use rand::Rng;
use std::env;

// para el timer
use std::{thread, time};
use std::time::Duration;

// para achivos
use std::fs::File;
use std::io::prelude::*;
use std::fs;

fn main() -> std::io::Result<()> {

	//guardamos en un archivo la imagen de fondo que teniamos anteriormente
	let mut file = File::create("old_wallpaper.txt")?;
	let old_wallpaper = wallpaper::get().unwrap().to_string();
	file.write_all(b"Your old wallpaper is: ")?;
	file.write_all( old_wallpaper.as_bytes() )?;

	let mut imagenes = Vec::new();		//Arreglo de imagenes

	let current_path = env::current_dir()?;
	//let mut current_path: String = env::current_dir()?.display().to_string() + "\\img";
	// println!("The current directory is {}", current_path.display());

	let paths = fs::read_dir("./img").unwrap();

	for path in paths {
		let path = path?;	//usar ? es como usar	.unwrap()

		// usar find() regresa una options (some), para lo cual debemos usar unwrap, pero unwrap sobre none o 0 muere asi ke usamos la version segura unwrap_or_default()
		if 0 != path.path().display().to_string().find("jpg").unwrap_or_default() || 0 != path.path().display().to_string().find("png").unwrap_or_default() {
			imagenes.push( path.path().display().to_string().replace("./", "") as String );
		}
	}

	let mut rng = thread_rng();

	let start_time = time::Instant::now();
	// println!("start_time {:?}", start_time);

	let timer = time::Duration::from_millis(5000);			//timer para cambiar de imagen

	// let mut game_loop = true;
	let game_loop = true;

	while game_loop {

		let posicion_random = rng.gen_range(0, imagenes.len());
		//println!("Number from 0 to {}: {}", imagenes.len(), posicion_random);

		let final_path: String = current_path.display().to_string() + "\\" + &imagenes[posicion_random];
		println!("Wallpaper to load: {}", &final_path);

		// wallpaper::set_from_url("https://source.unsplash.com/random").unwrap();
		wallpaper::set_from_path( &final_path ).unwrap();

		// println!("Wallpaper F{:?}", wallpaper::get());
		thread::sleep(timer);

		if Duration::new(60, 0) < start_time.elapsed(){
			// println!("paso mas de 1 minuto");
			// game_loop = false;
		}
	}

	// println!("Current {:?}", start_time.elapsed());

	if Duration::new(60, 0) < start_time.elapsed(){
		// println!("paso mas de 1 minuto");
	}

	Ok(())
}
