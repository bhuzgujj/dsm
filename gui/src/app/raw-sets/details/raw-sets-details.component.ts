import { Component, inject, Inject, input, model } from '@angular/core';
import { NgFor, NgIf } from "@angular/common";
import { DsmSets } from "../../../backend.types";
import { FormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { getRawSet } from '../../../backend';

type ClassCount = {
    class: string,
    subclass: string | null,
    count: number
};

@Component({
    selector: 'app-raw-sets-details',
    standalone: true,
    imports: [
        NgIf,
        NgFor,
        FormsModule
    ],
    templateUrl: './raw-sets-details.component.html'
})
export class RawSetsDetailsComponent {
    datasets: DsmSets | null | undefined = undefined;
    name = "";
    version = "";
    annotations: ClassCount[] = [];

    constructor(route: ActivatedRoute) {
        route.paramMap.subscribe((params) => {
            let name = params.get('name')!;
            let version = params.get('version')!;
            getRawSet(name, version)
                .then((ds) => {
                    this.datasets = ds;
                    if (ds) {
                        this.extractDataset(ds);
                    }
                })
                .catch((err) => {
                    this.datasets = null;
                    console.error(err);
                });
        });
    }

    private extractDataset(ds: DsmSets) {
        this.name = ds.metadata.name;
        this.version = ds.metadata.version;
        for (const classes in ds.metadata.classes) {
            this.annotations[classes] = {
                ...ds.metadata.classes[classes],
                count: 0
            };
        }
        for (const group in ds.entries) {
            for (const entry of ds.entries[group]) {
                for (const annotation of entry.annotation) {
                    this.annotations[annotation.class].count++;
                }
            }
        }
    }
}
