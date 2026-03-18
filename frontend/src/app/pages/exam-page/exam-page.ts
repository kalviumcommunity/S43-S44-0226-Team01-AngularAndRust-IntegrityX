import { Component, OnInit } from '@angular/core';
import { Api } from '../../services/api';
import { ActivatedRoute } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ChangeDetectorRef } from '@angular/core';
import { Router } from '@angular/router';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-exam-page',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './exam-page.html',
  styleUrl: './exam-page.css'
})
export class ExamPageComponent implements OnInit {
  examId: number = 0;
  timeLeft = 600; // 10 minutes
  timer: any;
  violations = 0;
  answer:string = ""; 
  examEnded = false;
  fullscreenExited = false;
  showViolationsTerminated = false;
  get formattedTime() {
    const minutes = Math.floor(this.timeLeft / 60);
    const seconds = this.timeLeft % 60;

    return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
  }

  constructor(
    private api: Api,
    private route: ActivatedRoute,
    private cd: ChangeDetectorRef,
    private router: Router,
  ) {}

  ngOnInit() {
    this.examId = Number(this.route.snapshot.paramMap.get('id'));
    this.startTimer();

    if (typeof document !== 'undefined') {
      const handleKeyDown = (e: KeyboardEvent) => {
        if (e.key === 'Escape') {
          e.preventDefault();
          e.stopPropagation();
        } else if (e.ctrlKey || e.metaKey) {
          e.preventDefault();
        }
      };
      document.addEventListener('keydown', handleKeyDown, true);
      document.documentElement.requestFullscreen();

      // On fullscreen exit: log and show overlay until user clicks to re-enter (browser only allows requestFullscreen on user gesture)
      document.addEventListener('fullscreenchange', () => {
        if (!document.fullscreenElement && !this.examEnded) {
          this.logEvent('fullscreen_exit');
          this.fullscreenExited = true;
          this.cd.detectChanges();
        }
      });

      // Detect tab switching
      document.addEventListener('visibilitychange', () => {
        if (document.hidden) {
          this.logEvent('tab_switch');
        }
      });

      // Detect window blur
      window.addEventListener('blur', () => {
        this.logEvent('window_blur');
      });

      // Detect copy attempt
      document.addEventListener('copy', (event) => {
        event.preventDefault();
        this.logEvent('copy_attempt');
      });
      document.addEventListener('paste', (event) => {
        event.preventDefault();
        this.logEvent('paste_attempt');
      });

      // Block right click
      document.addEventListener('contextmenu', (event) => {
        event.preventDefault();
        this.logEvent('right_click');
      });
    }
  }
  submitExam() {
    clearInterval(this.timer);

    this.api
      .submitExam({
        student_id: 1,
        exam_id: this.examId,
        score: 80,
      })
      .subscribe(() => {
        this.router.navigate(['/']); // go back to home
      });
  }
  startTimer() {
    this.timer = setInterval(() => {
      this.timeLeft--;

      this.cd.detectChanges(); // force UI update

      if (this.timeLeft <= 0) {
        clearInterval(this.timer);
        if (typeof window !== 'undefined') {
          alert("Too many violations. Exam terminated.");
        }
        this.submitExam();
      }
    }, 1000);
  }
  get timeLeftMinutes() {
    return Math.floor(this.timeLeft / 60);
  }

  get timeLeftSeconds() {
    return (this.timeLeft % 60).toString().padStart(2, '0');
  }

  reenterFullscreen() {
    document.documentElement.requestFullscreen().then(() => {
      this.fullscreenExited = false;
      this.cd.detectChanges();
    }).catch(() => {});
  }

  logEvent(event: string) {
    if (this.examEnded) return; // stop further violations

    this.violations++;

    this.cd.detectChanges();

    this.api
      .logActivity({
        student_id: 1,
        exam_id: this.examId,
        event_type: event,
      })
      .subscribe();

    if (this.violations >= 5) {
      this.examEnded = true;
      clearInterval(this.timer);
      this.showViolationsTerminated = true;
      this.cd.detectChanges();
    }
  }
}
