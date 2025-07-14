import { Component, inject, Inject, input, model } from '@angular/core';
import { NgFor, NgIf } from "@angular/common";
import { DsmSets, MergedSet } from "../../../backend.types";
import { FormsModule } from '@angular/forms';
import { ActivatedRoute } from '@angular/router';
import { generateMergedSet, getMergedSet, getRawSet } from '../../../backend';
import { open } from '@tauri-apps/plugin-dialog';

type ClassCount = {
    class: string,
    subclass: string | null,
    count: number
};

@Component({
    selector: 'app-merged-sets-details',
    standalone: true,
    imports: [
        NgIf,
        NgFor,
        FormsModule
    ],
    templateUrl: './merged-sets-details.component.html'
})
export class MergedSetsDetailsComponent {
    datasets: MergedSet | null | undefined = undefined;
    name = "";
    version = "";
    annotations: ClassCount[] = [];

    constructor(route: ActivatedRoute) {
        route.paramMap.subscribe((params) => {
            let name = params.get('name')!;
            let version = params.get('version')!;
            getMergedSet(name, version)
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

    generate() {
        open({
            multiple: false,
            directory: true
        })
            .then((path) => generateMergedSet(this.name, this.version, "yolo-1-1", path!))
            .then(() => window.location.href = "/merged-sets")
            .catch(console.error)
    }

    private extractDataset(ds: MergedSet) {
        this.name = ds.name;
        this.version = ds.version;
        for (const classes of Object.keys(ds.class_mapper.classes)) {
            const index = ds.class_mapper.classes[classes]
            this.annotations[index] = {
                class: classes,
                subclass: null,
                count: 0
            };
        }
        for (const group of Object.values(ds.datasets)) {
            for (const dsm of group) {
                for (const subset in dsm.entries) {
                    for (const entry of dsm.entries[subset]) {
                        for (const annotation of entry.annotation) {
                            const classKey = dsm.metadata.classes[annotation.class].class;
                            let mappedKey = null;
                            for (const keys in ds.class_mapper.mapping) {
                                if (ds.class_mapper.mapping[keys].includes(classKey)) {
                                    mappedKey = keys;
                                    break;
                                }
                            }
                            const mappedIndex = ds.class_mapper.classes[mappedKey!];
                            this.annotations[mappedIndex].count++;
                        }
                    }
                }
            }
        }
    }
}
