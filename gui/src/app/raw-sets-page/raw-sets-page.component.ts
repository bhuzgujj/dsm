import { Component } from '@angular/core';
import {listRawDatasets} from "../../backend";
import {NgForOf, NgIf} from "@angular/common";
import {DsmEntry, DsmSets} from "../../backend.types";

@Component({
  selector: 'app-raw-sets-page',
  standalone: true,
  imports: [
    NgForOf,
    NgIf
  ],
  templateUrl: './raw-sets-page.component.html',
  styleUrl: './raw-sets-page.component.css'
})
export class RawSetsPageComponent {
  datasets: DsmSets[] = []
  selected: DsmSets | null = null
  constructor() {
    listRawDatasets().then((sets) => {
      this.datasets = sets
    })
  }

  select(sets: DsmSets) {
    if (this.selected === sets)
      this.selected = null
    else
      this.selected = sets
  }

  countSet(sets: DsmSets, id: number): number {
    return Object.values(sets.entries).reduce((acc, val)=> {
      for(const entry of val)
        for(const annotation of entry.annotation)
          if (annotation.class === id)
            acc++;
      return acc;
    }, 0)
  }

  protected readonly Object = Object;
}
