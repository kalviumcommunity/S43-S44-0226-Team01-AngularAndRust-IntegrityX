import { Component } from '@angular/core';
import { Api } from '../../services/api';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators';

@Component({
  selector: 'app-exam-list',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './exam-list.html',
  styleUrl: './exam-list.css'
})
export class ExamListComponent {

  exams$: Observable<any[]>;

  constructor(
    private api: Api,
    private router: Router
  ) {
    this.exams$ = this.api.getExams().pipe(
      map((data: any) => {
        const list = Array.isArray(data) ? data : (data?.exams ?? []);
        console.log("EXAMS:", list);
        return list;
      }),
      catchError(() => of([]))
    );
  }

  startExam(id: number) {
    this.router.navigate(['/exam', id]);
  }

}