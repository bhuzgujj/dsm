import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RawSetsPageComponent } from './raw-sets-page.component';

describe('RawSetsPageComponent', () => {
  let component: RawSetsPageComponent;
  let fixture: ComponentFixture<RawSetsPageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RawSetsPageComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(RawSetsPageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
