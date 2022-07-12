class GameObject {
  x: number;
  y: number;
  width: number;
  height: number;
  color: string;
  constructor(x: number, y: number, width: number, height: number,
              color: string) {
    this.x = x;
    this.y = y;
    this.width = width;
    this.height = height;
    this.color = color;
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.fillStyle = this.color;
    ctx.fillRect(this.x, this.y, this.width, this.height);
  }

  update(dx: number, dy: number) {
    this.x += dx;
    this.y += dy;
  }

  collidesWith(obj: GameObject) {
    let inside_xaxis =
        this.x < (obj.x + obj.width) && obj.x < (this.x + this.width);
    let inside_yaxis =
        this.y < (obj.y + obj.height) && obj.y < (this.y + this.height);
    return (inside_xaxis && inside_yaxis);
  }
}

class Bullet extends GameObject {
  dy: number;
  constructor(x: number, y: number, width: number, height: number,
              color: string, dy: number) {
    super(x, y, width, height, color);
    this.dy = dy;
  }

  update(_dx: number, _dy: number) { this.y += this.dy; }
}

export class SpaceShip extends GameObject {
  canvasHeight: number;
  bulletWidth: number;
  bulletHeight: number;
  bulletColor: string;
  bullets: Bullet[];
  constructor(x: number, y: number, width: number, height: number,
              color: string, canvasHeight: number) {
    super(x, y, width, height, color);
    this.canvasHeight = canvasHeight;
    this.bulletWidth = 4;
    this.bulletHeight = 8;
    this.bulletColor = '#ff7800';
    this.bullets = [];
  }
  draw(ctx: CanvasRenderingContext2D) {
    super.draw(ctx);
    for (var i = 0; i < this.bullets.length; i++) {
      this.bullets[i].draw(ctx);
      this.bullets[i].update(0, 0);
      // Check if the bullet is out of bounds
      if (this.bullets[i].y < 0 || this.bullets[i].y > this.canvasHeight) {
        this.bullets.splice(i, 1);
      }
    }
  }

  shoot(_dy: number) {
    var x_pos = this.x + this.width / 2 - this.bulletWidth / 2;
    var y_pos = this.y - this.bulletHeight;
    this.bullets.push(new Bullet(x_pos, y_pos, this.bulletWidth,
                                 this.bulletHeight, this.bulletColor, 0))
  }
}

export class Player extends SpaceShip {
  canvasWidth: number;
  constructor(x: number, y: number, width: number, height: number,
              color: string, canvasHeight: number, canvasWidth: number) {
    super(x, y, width, height, color, canvasHeight);
    this.canvasWidth = canvasWidth;
  }

  // Update the player's position
  update(dx: number, dy: number) {
    super.update(dx, dy);
    // Keep the player within the canvas
    if (this.x < 0) {
      this.x = 0;
    } else if (this.x + this.width > this.canvasWidth) {
      this.x = this.canvasWidth - this.width;
    }
  }
}

export class Asteroid {
  parts: GameObject[];
  constructor(x: number, y: number, width: number, height: number,
              color: string, noParts: any) {
    this.parts = [];
    for (var i = 0; i < noParts; i++) {
      for (var j = 0; j < noParts; j++) {
        var x_pos = x + i * width;
        var y_pos = y + j * height;
        this.parts.push(new GameObject(x_pos, y_pos, width, height, color));
      }
    }
  }

  // Draw the asteroid on the canvas
  draw(ctx: CanvasRenderingContext2D) {
    for (var i = 0; i < this.parts.length; i++) {
      this.parts[i].draw(ctx);
    }
  }

  // Check if the asteroid is colliding with another
  // object
  collidesWith(obj: GameObject) {
    for (var i = 0; i < this.parts.length; i++) {
      if (this.parts[i].collidesWith(obj)) {
        return true;
      }
    }
    return false;
  }

  // Remove sub object on collide
  removeOnCollide(obj: GameObject) {
    for (var i = 0; i < this.parts.length; i++) {
      if (this.parts[i].collidesWith(obj)) {
        this.parts.splice(i, 1);
        break;
      }
    }
  }
}
