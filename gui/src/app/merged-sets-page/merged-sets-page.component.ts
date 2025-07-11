import { Component } from '@angular/core';
import {NgForOf, NgIf} from "@angular/common";
import {MergedSet} from "../../backend.types";
import {listMergedDatasets} from "../../backend";

@Component({
  selector: 'app-merged-sets-page',
  standalone: true,
    imports: [
        NgForOf,
        NgIf
    ],
  templateUrl: './merged-sets-page.component.html',
  styleUrl: './merged-sets-page.component.css'
})
export class MergedSetsPageComponent {
    datasets: MergedSet[] = []
    selected: MergedSet | null = null
    constructor() {
        listMergedDatasets().then((sets) => {
            this.datasets = sets
        })
    }

    select(sets: MergedSet) {
        if (this.selected === sets)
            this.selected = null
        else
            this.selected = sets
    }

    protected readonly Object = Object;
}
