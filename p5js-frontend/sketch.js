class Boid {
  constructor(pos, dir, spd, wht, prc, sep, aln, coh) {
    this.position = pos;
    this.direction = dir;

    this.speed = spd;
    this.weight = wht;

    this.radius_perception = prc;
    this.force_seperation = sep;
    this.force_alignment = aln;
    this.force_cohesion = coh;
  }
}


function setup() {
  frameRate(30);
  createCanvas(800, 800);

  let flock = [];
  for (let i = 0; i < 100; i++) {
    flock.push(new Boid([random(0, width), random(0, height)], [0, 0], 3, random(7, 15), 100, 1, 1, 1));
  }
  console.log(flock);

  wasm_bindgen.flock_add_boids(flock);
}


function draw() {
  background(50);
  wasm_bindgen.flock_update();

  let flock = wasm_bindgen.flock_get();
  for (boid of flock) {
    fill(255);
    stroke(255);
    strokeWeight(0);
    circle(boid.position[0], boid.position[1], boid.weight);

    stroke(100);
    strokeWeight(1)
    noFill();
    circle(boid.position[0], boid.position[1], boid.radius_perception * 2);
  }
}
