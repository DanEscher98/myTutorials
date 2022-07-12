import {Asteroid, Player, SpaceShip} from './gameObjects';
import gameSettings from './gameSettings.json';

class GameLoop {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;
  interval: number;
  player: Player;
  asteroids: Asteroid[];
  enemies: SpaceShip[];
  enemyDirection: number;
  enemyFireTimer: number;
  constructor() {
    this.canvas = document.getElementById('canvas') as HTMLCanvasElement;
    this.ctx = this.canvas.getContext("2d")!;
    this.interval = 0;
    this.player =
        new Player(this.canvas.width / 2 - 50, this.canvas.height - 50, 20, 20,
                   '#0099CC', this.canvas.height, this.canvas.width);
    this.asteroids = [];
    this.enemies = [];
    this.enemyDirection = 1;
    this.enemyFireTimer = 0;
  }
  init() {
    this.interval = setInterval(this.update, 1000 / 60);

    // Setup asteroids
    for (var i = 0; i < gameSettings.noOfAsteroids; i++) {
      var x_pos = gameSettings.asteroidsSpace + i * gameSettings.asteroidsSpace;
      var y_pos = this.canvas.height - 180;
      this.asteroids.push(new Asteroid(x_pos, y_pos, 5, 5, '#ffffff',
                                       gameSettings.asteroidsParts));
    }

    // Setup enemies
    for (var i = 0; i < gameSettings.enemyLines; i++) {
      for (var j = 0; j < gameSettings.enemiesEachLine; j++) {
        var x_pos = gameSettings.enemySpace + j * gameSettings.enemySpace;
        var y_pos = gameSettings.enemySpace + i * gameSettings.enemySpace;
        this.enemies.push(
            new SpaceShip(x_pos, y_pos, 20, 20, '#FF0000', this.canvas.height));
      }
    }
  }
  keydown(e: KeyboardEvent) {
    if (e.key == 'Left') {
      this.player.update(-5, 0);
    } else if (e.key == 'Right') {
      this.player.update(5, 0);
    } else if (e.key == 'Space') {
      this.player.shoot(-5);
    }
  }
  update() {
    // Draw canvas background
    this.ctx.fillStyle = gameSettings.backgroundColor;
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

    // Draw player
    this.player.draw(this.ctx);

    // Draw asteroids
    for (var i = 0; i < this.asteroids.length; i++) {
      this.asteroids[i].draw(this.ctx);
    }

    // Draw enemies
    for (var i = 0; i < this.enemies.length; i++) {
      this.enemies[i].draw(this.ctx);
      this.enemies[i].update(this.enemyDirection, 0);
    }

    // Check if the player has destroyed all enemies
    if (this.enemies.length == 0) {
      this.restart();
    }

    // Check if the enemies are out of bounds
    if (this.enemyDirection == 1) {
      // Find the enemy closest to the right side of the
      // screen
      var closestToRightSideEnemy = this.enemies[0];
      for (var i = 1; i < this.enemies.length; i++) {
        if (game.enemies[i].x > closestToRightSideEnemy.x) {
          closestToRightSideEnemy = game.enemies[i];
        }
      }

      // Check if the enemy closest to the right side of
      // the screen has reached the right side of the
      // screen
      if (closestToRightSideEnemy.x + closestToRightSideEnemy.width >
          this.canvas.width) {
        this.enemyDirection = -1;
        // Move the enemies down
        for (var i = 0; i < game.enemies.length; i++) {
          this.enemies[i].update(0, gameSettings.enemyStep);
        }
      }
    } else {
      // Find the enemy closest to the left
      var closestToLeftSideEnemy = game.enemies[0];
      for (var i = 0; i < game.enemies.length; i++) {
        if (this.enemies[i].x < closestToLeftSideEnemy.x) {
          closestToLeftSideEnemy = this.enemies[i];
        }
      }

      // Check if the enemy closest to the left side has
      // reached the left side of the screen
      if (closestToLeftSideEnemy.x < 0) {
        this.enemyDirection = 1;
        for (var i = 0; i < game.enemies.length; i++) {
          this.enemies[i].update(0, gameSettings.enemyStep);
        }
      }
    }

    // Enemy fire counter
    this.enemyFireTimer += Math.random() * 10;
    if (this.enemyFireTimer > gameSettings.enemyFireRate) {
      this.enemyFireTimer = 0;
      // Fire enemy bullet
      this.enemies[Math.floor(Math.random()) * this.enemies.length].shoot(5);
    }

    // Check if player bullet collides with asteroid
    for (var i = 0; i < this.player.bullets.length; i++) {
      for (var j = 0; j < this.asteroids.length; j++) {
        if (this.asteroids[j].collidesWith(this.player.bullets[i])) {
          this.asteroids[j].removeOnCollide(this.player.bullets[i]);
          this.player.bullets.splice(i, 1);
          break;
        }
      }
    }

    // Check if enemy bullet collides with asteroid
    for (var i = 0; i < this.enemies.length; i++) {
      for (var j = 0; j < this.enemies[i].bullets.length; j++) {
        for (var k = 0; k < this.asteroids.length; k++) {
          if (this.asteroids[k].collidesWith(this.enemies[i].bullets[j])) {
            this.asteroids[k].removeOnCollide(this.enemies[i].bullets[j]);
            this.enemies[i].bullets.splice(j, 1);
            break;
          }
        }
      }
    }

    // Check if player bullet collides with enemy
    for (var i = 0; i < this.player.bullets.length; i++) {
      for (var j = 0; j < this.enemies.length; j++) {
        if (this.enemies[j].collidesWith(this.player.bullets[i])) {
          this.enemies.splice(j, 1);
          this.player.bullets.splice(i, 1);
          break;
        }
      }
    }

    // Check if enemy bullet collides with player
    for (var i = 0; i < this.enemies.length; i++) {
      for (var j = 0; j < this.enemies[i].bullets.length; j++) {
        if (this.player.collidesWith(this.enemies[i].bullets[j])) {
          // Reset the game
          this.restart();
          break;
        }
      }
    }

    // Check if an enemy has reached the player's y position
    for (var i = 0; i < this.enemies.length; i++) {
      if (this.enemies[i].y + this.enemies[i].height > this.player.y) {
        this.restart();
        break;
      }
    }
  }

  stop() { clearInterval(this.interval); }
  restart() {
    this.stop();
    this.init();
  }
};

var game = new GameLoop;
// Start the game on window load
window.onload = game.init;
// Detect key events
window.onkeydown = game.keydown;
