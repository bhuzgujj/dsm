import { Component } from '@angular/core';
import { MergedSet } from "../../backend.types";
import { listMergedDatasets } from "../../backend";
import { RouterLink, RouterOutlet } from '@angular/router';

@Component({
    selector: 'app-merged-sets-page',
    standalone: true,
    imports: [
        RouterOutlet,
        RouterLink
    ],
    templateUrl: './merged-sets-page.component.html'
})
export class MergedSetsPageComponent {
}
