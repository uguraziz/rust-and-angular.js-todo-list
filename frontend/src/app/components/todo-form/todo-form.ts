import { Component, EventEmitter, Output } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { CreateTodo } from '../../models/todo.model';

@Component({
  selector: 'app-todo-form',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './todo-form.html',
  styleUrls: ['./todo-form.css']
})
export class TodoFormComponent {
  @Output() todoCreated = new EventEmitter<CreateTodo>();

  title: string = '';
  description: string = '';

  onSubmit(): void {
    if (this.title.trim()) {
      const newTodo: CreateTodo = {
        title: this.title.trim(),
        description: this.description.trim() || undefined
      };

      this.todoCreated.emit(newTodo);
      this.resetForm();
    }
  }

  resetForm(): void {
    this.title = '';
    this.description = '';
  }
}