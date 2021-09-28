import {Component, Input, OnInit} from '@angular/core';
import { PlayedTile } from '../shared/played-tile';
import { Task } from '../shared/task';

@Component({
  selector: 'app-tile',
  templateUrl: './tile.component.html',
  styleUrls: ['./tile.component.scss']
})
export class TileComponent {

  @Input()
  tile?: PlayedTile;

  getTask(num: number): Task | undefined {
    let tasks = this.tile?.tile?.tasks;
    if (tasks) {
      let len = tasks.length;
      if (num >= 0 && num < len) {
        return tasks[num];
      }
    }
    return undefined;
  }

  position(): string {
    let position = this.tile?.position;
    let size = 256;
    if (position != null) {
      return `top:${position[0] * size}px;left:${position[1] * size}px;`;
    }
    return `top:0px;left:0px;`;
  }
}
