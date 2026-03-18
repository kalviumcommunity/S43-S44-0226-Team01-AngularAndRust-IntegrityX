import { Routes } from '@angular/router';
import { ExamListComponent } from './pages/exam-list/exam-list';
import { ExamPageComponent } from './pages/exam-page/exam-page';

export const routes: Routes = [
  { path: '', component: ExamListComponent },
  { path: 'exam/:id', component: ExamPageComponent }
];