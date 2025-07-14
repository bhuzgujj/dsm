import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { ClassMapper, DsmSets, IncludedSet, MergedSet } from '../../../backend.types';
import { listMergedDatasets, listRawDatasets, mergeNew } from '../../../backend';
import { NgFor, NgIf } from '@angular/common';

type Mapping = {
    from: string,
    to: string | null
}

@Component({
    selector: 'app-merged-sets-create',
    standalone: true,
    imports: [
        FormsModule,
        NgFor,
        NgIf,
    ],
    templateUrl: './merged-sets-create.component.html'
})
export class MergedSetsCreateComponent {
    baseSet: MergedSet | null = null;
    mergedSets: MergedSet[] = [];
    rawSets: DsmSets[] = [];
    included: IncludedSet[] = [];
    groups: string[] = [];
    classes: string[] = [];

    name = "";
    version = "";

    // Sub form
    inputGroup = "";
    addedGroup = "";
    addedSets: DsmSets | null = null;
    newClass = "";

    mapping: Mapping[] = []

    constructor() {
        listMergedDatasets().then((sets) => this.mergedSets = sets);
        listRawDatasets().then((sets) => this.rawSets = sets);
    }

    selectMerged(merged: MergedSet) {
        this.baseSet = merged;
    }

    addGroup(group: string) {
        if (!this.groups.includes(group))
            this.groups.push(group)
    }

    addSet(group: string, dataset: DsmSets | null) {
        if (dataset !== null) {
            const names = dataset.metadata.name;
            const version = dataset.metadata.version;
            let found = false;
            for (const included of this.included) {
                if (included.names === names && included.version === version) {
                    included.group = group;
                    found = true;
                    break
                }
            }
            if (!found) {
                this.included.push({
                    names,
                    version,
                    group
                })
                for (const dsmClasses of Object.values(dataset.metadata.classes)) {
                    const cla = dsmClasses.class;
                    let containClass = false;
                    for (const mapping of this.mapping) {
                        if (mapping.from === cla) {
                            containClass = true;
                            break;
                        }
                    }
                    if (!containClass) {
                        let mapTo = null;
                        for (const classs of this.classes) {
                            if (classs === cla) {
                                mapTo = classs;
                                break;
                            }
                        }
                        this.mapping.push({
                            from: cla,
                            to: mapTo
                        })
                    }
                }
            }
        }
    }

    changeMapTo(from: string, to: any | null) {
        if (to) {
            for (const mapping of this.mapping) {
                if (mapping.from === from) {
                    mapping.to = to.value
                }
            }
        }
    }

    createMapping() {
        const map: ClassMapper = {
            classes: {},
            mapping: {},
            custom: null
        };
        for (let index = 0; index < this.classes.length; index++) {
            map.classes[this.classes[index]] = index;
        }
        for (const mapping of this.mapping) {
            if (mapping.to === null || mapping.to === undefined || map.classes[mapping.to] === undefined) {
                return null;
            }
            if (map.mapping[mapping.to] === undefined) {
                map.mapping[mapping.to] = [];
            }
            map.mapping[mapping.to] = [...map.mapping[mapping.to], mapping.from];
        }

        return map;
    }

    addClasses(classs: string) {
        for (const c of this.classes) {
            if (classs === c) {
                return;
            }
        }
        this.classes.push(classs);
        for (const mapping of this.mapping) {
            if (mapping.to === null && mapping.from === classs) {
                mapping.to = classs;
            }
        }
    }

    store() {
        const mapping = this.createMapping();
        if (mapping === null)
            return;
        if (this.baseSet !== null) {
            console.log(this)
        } else {
            mergeNew(this.name, this.version, this.included, mapping)
        }
    }

    protected readonly Object = Object;
}

