import { Task } from "./task";
export interface Tile {
  cost: string;
  color: string;
  tasks: Task[];
}
