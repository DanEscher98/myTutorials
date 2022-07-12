"use strict";
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.Asteroid = exports.Player = exports.SpaceShip = void 0;
var GameObject = /** @class */ (function () {
    function GameObject(x, y, width, height, color) {
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
        this.color = color;
    }
    GameObject.prototype.draw = function (ctx) {
        ctx.fillStyle = this.color;
        ctx.fillRect(this.x, this.y, this.width, this.height);
    };
    GameObject.prototype.update = function (dx, dy) {
        this.x += dx;
        this.y += dy;
    };
    GameObject.prototype.collidesWith = function (obj) {
        var inside_xaxis = this.x < (obj.x + obj.width) && obj.x < (this.x + this.width);
        var inside_yaxis = this.y < (obj.y + obj.height) && obj.y < (this.y + this.height);
        return (inside_xaxis && inside_yaxis);
    };
    return GameObject;
}());
var Bullet = /** @class */ (function (_super) {
    __extends(Bullet, _super);
    function Bullet(x, y, width, height, color, dy) {
        var _this = _super.call(this, x, y, width, height, color) || this;
        _this.dy = dy;
        return _this;
    }
    Bullet.prototype.update = function (_dx, _dy) { this.y += this.dy; };
    return Bullet;
}(GameObject));
var SpaceShip = /** @class */ (function (_super) {
    __extends(SpaceShip, _super);
    function SpaceShip(x, y, width, height, color, canvasHeight) {
        var _this = _super.call(this, x, y, width, height, color) || this;
        _this.canvasHeight = canvasHeight;
        _this.bulletWidth = 4;
        _this.bulletHeight = 8;
        _this.bulletColor = '#ff7800';
        _this.bullets = [];
        return _this;
    }
    SpaceShip.prototype.draw = function (ctx) {
        _super.prototype.draw.call(this, ctx);
        for (var i = 0; i < this.bullets.length; i++) {
            this.bullets[i].draw(ctx);
            this.bullets[i].update(0, 0);
            // Check if the bullet is out of bounds
            if (this.bullets[i].y < 0 || this.bullets[i].y > this.canvasHeight) {
                this.bullets.splice(i, 1);
            }
        }
    };
    SpaceShip.prototype.shoot = function (_dy) {
        var x_pos = this.x + this.width / 2 - this.bulletWidth / 2;
        var y_pos = this.y - this.bulletHeight;
        this.bullets.push(new Bullet(x_pos, y_pos, this.bulletWidth, this.bulletHeight, this.bulletColor, 0));
    };
    return SpaceShip;
}(GameObject));
exports.SpaceShip = SpaceShip;
var Player = /** @class */ (function (_super) {
    __extends(Player, _super);
    function Player(x, y, width, height, color, canvasHeight, canvasWidth) {
        var _this = _super.call(this, x, y, width, height, color, canvasHeight) || this;
        _this.canvasWidth = canvasWidth;
        return _this;
    }
    // Update the player's position
    Player.prototype.update = function (dx, dy) {
        _super.prototype.update.call(this, dx, dy);
        // Keep the player within the canvas
        if (this.x < 0) {
            this.x = 0;
        }
        else if (this.x + this.width > this.canvasWidth) {
            this.x = this.canvasWidth - this.width;
        }
    };
    return Player;
}(SpaceShip));
exports.Player = Player;
var Asteroid = /** @class */ (function () {
    function Asteroid(x, y, width, height, color, noParts) {
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
    Asteroid.prototype.draw = function (ctx) {
        for (var i = 0; i < this.parts.length; i++) {
            this.parts[i].draw(ctx);
        }
    };
    // Check if the asteroid is colliding with another
    // object
    Asteroid.prototype.collidesWith = function (obj) {
        for (var i = 0; i < this.parts.length; i++) {
            if (this.parts[i].collidesWith(obj)) {
                return true;
            }
        }
        return false;
    };
    // Remove sub object on collide
    Asteroid.prototype.removeOnCollide = function (obj) {
        for (var i = 0; i < this.parts.length; i++) {
            if (this.parts[i].collidesWith(obj)) {
                this.parts.splice(i, 1);
                break;
            }
        }
    };
    return Asteroid;
}());
exports.Asteroid = Asteroid;
