import { Routes } from "@angular/router";
import { RawSetsPageComponent } from "./raw-sets/page.component";
import { MergedSetsPageComponent } from "./merged-sets/page.component";
import { RawSetsListComponent } from "./raw-sets/list.component";
import { RawSetsCreateComponent } from "./raw-sets/create.component";
import { HomePageComponent } from "./home/page.component";
import { RawSetsDetailsComponent } from "./raw-sets/details.component";
import { MergedSetsListComponent } from "./merged-sets/list.component";
import { MergedSetsCreateComponent } from "./merged-sets/create/create.component";
import { MergedSetsDetailsComponent } from "./merged-sets/details.component";

export const routes: Routes = [
    {
        path: "raw-sets",
        component: RawSetsPageComponent,
        children: [
            {
                path: "",
                component: RawSetsListComponent,
                pathMatch: 'full'
            },
            {
                path: "create",
                component: RawSetsCreateComponent
            },
            {
                path: ":name/:version",
                component: RawSetsDetailsComponent
            }
        ]
    },
    {
        path: "merged-sets",
        component: MergedSetsPageComponent,
        children: [
            {
                path: "",
                component: MergedSetsListComponent,
                pathMatch: 'full'
            },
            {
                path: "create",
                component: MergedSetsCreateComponent
            },
            {
                path: ":name/:version",
                component: MergedSetsDetailsComponent
            }
        ]
    },
    {
        path: '**',
        component: HomePageComponent
    },
];
