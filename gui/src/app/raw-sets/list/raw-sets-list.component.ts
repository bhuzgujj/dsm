import { Component, Inject } from '@angular/core';
import { listRawDatasets } from "../../../backend";
import { DOCUMENT, NgForOf, NgIf } from "@angular/common";
import { DsmSets } from "../../../backend.types";

@Component({
    selector: 'app-raw-sets-list',
    standalone: true,
    imports: [
        NgForOf,
        NgIf
    ],
    templateUrl: './raw-sets-list.component.html'
})
export class RawSetsListComponent {
    datasets: DsmSets[] = [];

    constructor() {
        listRawDatasets().then((sets) => this.datasets = sets);
    }

    select(sets: DsmSets) {
        window.location.href = `/raw-sets/${sets.metadata.name}/${sets.metadata.version}`
    }

    formatClasses(sets: DsmSets): string {
        return Object.values(sets.metadata.classes).reduce((acc, val) => {
            if (acc.length > 0)
                acc = acc + ", " + val.class;
            else
                acc = val.class;
            if (val.subclass)
                acc = acc + val.class + "(" + val.subclass + ")";
            return acc;
        }, "");
    }

    countImage(sets: DsmSets): number {
        return Object.values(sets.entries).reduce((acc, val) => {
            for (const _ of val)
                acc++;
            return acc;
        }, 0);
    }

    protected readonly Object = Object;
}
