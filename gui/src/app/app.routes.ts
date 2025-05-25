import { Routes } from "@angular/router";
import { RawSetsPageComponent } from "./raw-sets-page/raw-sets-page.component";
import { MergedSetsPageComponent } from "./merged-sets-page/merged-sets-page.component";

export const routes: Routes = [
    {
        path: "raw-sets",
        component: RawSetsPageComponent
    },
    {
        path: "merged-sets",
        component: MergedSetsPageComponent
    }
];
