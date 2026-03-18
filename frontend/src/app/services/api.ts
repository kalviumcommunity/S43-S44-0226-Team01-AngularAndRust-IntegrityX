import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class Api {

  baseUrl = "http://127.0.0.1:3000/api";

  constructor(private http: HttpClient) {}

  getExams(){
    return this.http.get(`${this.baseUrl}/exams`);
  }

  logActivity(data:any){
    return this.http.post(`${this.baseUrl}/activity`, data);
  }

  submitExam(data:any){
    return this.http.post(`${this.baseUrl}/submissions`, data);
  }

}