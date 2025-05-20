import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MergedSetsPageComponent } from './merged-sets-page.component';

describe('MergedSetsPageComponent', () => {
  let component: MergedSetsPageComponent;
  let fixture: ComponentFixture<MergedSetsPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MergedSetsPageComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MergedSetsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
