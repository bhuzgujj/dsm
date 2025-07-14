import { Component } from '@angular/core';
import { listMergedDatasets } from "../../../backend";
import { NgForOf, NgIf } from "@angular/common";
import { DsmSets, MergedSet } from "../../../backend.types";

@Component({
    selector: 'app-merged-sets-list',
    standalone: true,
    imports: [
        NgForOf,
        NgIf
    ],
    templateUrl: './merged-sets-list.component.html'
})
export class MergedSetsListComponent {
    datasets: MergedSet[] = [];

    constructor() {
        listMergedDatasets().then((sets) => this.datasets = sets);
    }

    select(sets: MergedSet) {
        window.location.href = `/merged-sets/${sets.name}/${sets.version}`
    }

    formatClasses(sets: MergedSet): string {
        return Object.keys(sets.class_mapper.classes).reduce((acc, val) => {
            if (acc.length > 0)
                acc = acc + ", " + val;
            else
                acc = val;
            return acc;
        }, "");
    }

    countDsm(sets: DsmSets): number {
        return Object.values(sets.entries).reduce((acc, val) => {
            for (const _ of val)
                acc++;
            return acc;
        }, 0);
    }

    countImage(sets: MergedSet): number {
        let count = 0;
        for (const group in sets.datasets) {
            count += sets.datasets[group]
                .map(this.countDsm)
                .reduce((acc, dsm) => acc + dsm);
        }
        return count;
    }

    protected readonly Object = Object;
}
