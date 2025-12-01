import { Component, OnInit, ChangeDetectorRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Todo, CreateTodo } from '../../models/todo.model';
import { TodoService } from '../../services/todo';
import { TodoItemComponent } from '../todo-item/todo-item';
import { TodoFormComponent } from '../todo-form/todo-form';

@Component({
  selector: 'app-todo-list',
  standalone: true,
  imports: [CommonModule, TodoItemComponent, TodoFormComponent],
  templateUrl: './todo-list.html',
  styleUrls: ['./todo-list.css']
})
export class TodoListComponent implements OnInit {
  todos: Todo[] = [];
  loading: boolean = true;
  error: string | null = null;

  constructor(
    private todoService: TodoService,
    private cdr: ChangeDetectorRef
  ) {}

  ngOnInit(): void {
    this.loadTodos();
  }

  loadTodos(): void {
    this.loading = true;
    this.error = null;
    this.cdr.detectChanges();

    this.todoService.getAllTodos().subscribe({
      next: (todos) => {
        this.todos = todos;
        this.loading = false;
        this.cdr.detectChanges();
      },
      error: (err) => {
        this.error = 'Todolar yüklenirken bir hata oluştu.';
        this.loading = false;
        this.cdr.detectChanges();
      },
      complete: () => {
        this.loading = false;
        this.cdr.detectChanges();
      }
    });
  }

  onTodoCreated(newTodo: CreateTodo): void {
    this.todoService.createTodo(newTodo).subscribe({
      next: (todo) => {
        this.todos.unshift(todo);
        this.cdr.detectChanges();
      },
      error: (err) => {
        alert('Todo oluşturulurken bir hata oluştu.');
      }
    });
  }

  onToggleTodo(id: number): void {
    this.todoService.toggleTodo(id).subscribe({
      next: (updatedTodo) => {
        const index = this.todos.findIndex(t => t.id === id);
        if (index !== -1) {
          this.todos[index] = updatedTodo;
          this.cdr.detectChanges();
        }
      },
      error: (err) => {
        alert('Todo durumu değiştirilirken bir hata oluştu.');
      }
    });
  }

  onDeleteTodo(id: number): void {
    this.todoService.deleteTodo(id).subscribe({
      next: () => {
        this.todos = this.todos.filter(t => t.id !== id);
        this.cdr.detectChanges();
      },
      error: (err) => {
        alert('Todo silinirken bir hata oluştu.');
      }
    });
  }

  get completedCount(): number {
    return this.todos.filter(t => t.completed).length;
  }

  get totalCount(): number {
    return this.todos.length;
  }
}