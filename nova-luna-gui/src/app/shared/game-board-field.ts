import { Tile } from "./tile";

export interface GameBoardField {
  position: number[];
  tile?: Tile;
}
