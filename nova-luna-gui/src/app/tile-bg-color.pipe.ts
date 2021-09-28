import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'tileBgColor'
})
export class TileBgColorPipe implements PipeTransform {
  transform(color: string | undefined, ...args: unknown[]): string {
    switch (color) {
      case "Blue":
        return "bg-blue-700"
      case "Yellow":
        return "bg-yellow-300 text-gray-800"
      case "Teal":
        return "bg-green-400";
      case "Red":
        return "bg-red-500";
    }
    return "bg-white-100";
  }
}
