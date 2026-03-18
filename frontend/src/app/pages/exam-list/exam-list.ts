import { Component, OnInit, ChangeDetectorRef } from '@angular/core';
import { Api } from '../../services/api';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-exam-list',
  standalone: true,
  imports: [CommonModule, RouterModule],
  templateUrl: './exam-list.html'
})
export class ExamListComponent implements OnInit {

  exams: any[] = [];

  constructor(
    private api: Api,
    private router: Router,
    private cdr: ChangeDetectorRef
  ) {}

  ngOnInit() {
    this.api.getExams().subscribe((data: any) => {
      console.log("EXAMS:", data);
      this.exams = Array.isArray(data) ? data : (data?.exams ?? []);
      this.cdr.detectChanges();
    });
  }

  startExam(id: number) {
    this.router.navigate(['/exam', id]);
  }

}