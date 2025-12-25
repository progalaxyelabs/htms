# Example: Todo Application

A complete todo application with add, edit, delete, and filter functionality.

## Features

- ✅ Add new todos
- ✅ Mark todos as complete
- ✅ Edit todo text (double-click)
- ✅ Delete todos
- ✅ Filter (All, Active, Completed)
- ✅ Clear completed todos
- ✅ Persistent storage (localStorage)

## Complete Source Code

### app.htms

```htms
component TodoItem(item: todo) {
  li [class: todo.done ? "todo-item completed" : "todo-item"] {
    div [class: "todo-content"] {
      input [
        type: "checkbox",
        checked: todo.done,
        onChange: toggleTodo(todo.id),
        class: "todo-checkbox"
      ]

      div @if(ctx.editingId == todo.id) {
        input [
          type: "text",
          value: todo.text,
          onBlur: saveEdit(todo.id),
          onKeydown: handleEditKeydown(todo.id),
          class: "todo-edit-input"
        ]
      } @else {
        span [
          class: "todo-text",
          onDblClick: startEdit(todo.id)
        ] {
          {{ ${todo.text} }}
        }
      }
    }

    button [
      class: "btn-delete",
      onClick: deleteTodo(todo.id),
      title: "Delete todo"
    ] {
      span {{ × }}
    }
  }
}

component TodoFilters {
  div [class: "filters"] {
    button [
      class: ctx.filter == "all" ? "filter-btn active" : "filter-btn",
      onClick: setFilter("all")
    ] {
      {{ All }}
    }

    button [
      class: ctx.filter == "active" ? "filter-btn active" : "filter-btn",
      onClick: setFilter("active")
    ] {
      {{ Active }}
    }

    button [
      class: ctx.filter == "completed" ? "filter-btn active" : "filter-btn",
      onClick: setFilter("completed")
    ] {
      {{ Completed }}
    }
  }
}

component TodoStats {
  div [class: "todo-stats"] {
    span [class: "items-left"] {
      {{ ${ctx.activeCount} ${ctx.activeCount == 1 ? "item" : "items"} left }}
    }

    button @if(ctx.completedCount > 0) [
      class: "btn-clear",
      onClick: clearCompleted
    ] {
      {{ Clear completed }}
    }
  }
}

page todos "/" {
  div [class: "todo-app"] {
    header [class: "app-header"] {
      h1 {{ todos }}
    }

    form [onSubmit.prevent: addTodo, class: "add-todo-form"] {
      input [
        type: "text",
        bind: ctx.newTodoText,
        placeholder: "What needs to be done?",
        class: "new-todo-input"
      ]
      button [
        type: "submit",
        disabled: ctx.newTodoText.length == 0,
        class: "btn-add"
      ] {
        {{ Add }}
      }
    }

    div @if(ctx.todos.length > 0) [class: "todos-container"] {
      TodoFilters

      ul [class: "todo-list"] {
        @each ctx.filteredTodos as todo {
          TodoItem(item: todo)
        }
      }

      TodoStats
    } @else {
      div [class: "empty-state"] {
        p {{ No todos yet. Add one above! }}
      }
    }
  }
}
```

### actions.ts

```typescript
export interface Todo {
  id: number;
  text: string;
  done: boolean;
  createdAt: number;
}

export interface TodoContext {
  todos: Todo[];
  newTodoText: string;
  filter: 'all' | 'active' | 'completed';
  editingId: number | null;
  editingText: string;
  filteredTodos: Todo[];
  activeCount: number;
  completedCount: number;
}

// Helper to save to localStorage
function saveTodos(todos: Todo[]) {
  localStorage.setItem('htms-todos', JSON.stringify(todos));
}

// Helper to compute filtered todos
function getFilteredTodos(todos: Todo[], filter: string): Todo[] {
  if (filter === 'active') return todos.filter(t => !t.done);
  if (filter === 'completed') return todos.filter(t => t.done);
  return todos;
}

// Helper to update context with computed values
function updateContext(ctx: any) {
  ctx.data.filteredTodos = getFilteredTodos(ctx.data.todos, ctx.data.filter);
  ctx.data.activeCount = ctx.data.todos.filter((t: Todo) => !t.done).length;
  ctx.data.completedCount = ctx.data.todos.filter((t: Todo) => t.done).length;
  saveTodos(ctx.data.todos);
  ctx.rerender();
}

export const actions = {
  // Add new todo
  addTodo: (ctx: any, event: Event) => {
    const text = ctx.data.newTodoText.trim();
    if (!text) return;

    const newTodo: Todo = {
      id: Date.now(),
      text,
      done: false,
      createdAt: Date.now()
    };

    ctx.data.todos.push(newTodo);
    ctx.data.newTodoText = '';
    updateContext(ctx);
  },

  // Toggle todo completion
  toggleTodo: (id: number) => (ctx: any, event: Event) => {
    const todo = ctx.data.todos.find((t: Todo) => t.id === id);
    if (todo) {
      todo.done = !todo.done;
      updateContext(ctx);
    }
  },

  // Delete todo
  deleteTodo: (id: number) => (ctx: any, event: Event) => {
    ctx.data.todos = ctx.data.todos.filter((t: Todo) => t.id !== id);
    updateContext(ctx);
  },

  // Start editing
  startEdit: (id: number) => (ctx: any, event: Event) => {
    const todo = ctx.data.todos.find((t: Todo) => t.id === id);
    if (todo) {
      ctx.data.editingId = id;
      ctx.data.editingText = todo.text;
      ctx.rerender();

      // Focus input after render
      setTimeout(() => {
        const input = document.querySelector('.todo-edit-input') as HTMLInputElement;
        if (input) {
          input.focus();
          input.select();
        }
      }, 0);
    }
  },

  // Save edit
  saveEdit: (id: number) => (ctx: any, event: Event) => {
    const input = event.target as HTMLInputElement;
    const newText = input.value.trim();

    if (newText) {
      const todo = ctx.data.todos.find((t: Todo) => t.id === id);
      if (todo) {
        todo.text = newText;
      }
    } else {
      // Delete if empty
      ctx.data.todos = ctx.data.todos.filter((t: Todo) => t.id !== id);
    }

    ctx.data.editingId = null;
    updateContext(ctx);
  },

  // Handle keyboard in edit mode
  handleEditKeydown: (id: number) => (ctx: any, event: KeyboardEvent) => {
    if (event.key === 'Enter') {
      (event.target as HTMLInputElement).blur();
    } else if (event.key === 'Escape') {
      ctx.data.editingId = null;
      ctx.rerender();
    }
  },

  // Set filter
  setFilter: (filter: 'all' | 'active' | 'completed') => (ctx: any, event: Event) => {
    ctx.data.filter = filter;
    updateContext(ctx);
  },

  // Clear completed
  clearCompleted: (ctx: any, event: Event) => {
    ctx.data.todos = ctx.data.todos.filter((t: Todo) => !t.done);
    updateContext(ctx);
  }
};
```

### main.ts

```typescript
import { setContext, router } from './dist/router';
import { initEvents } from './dist/events';
import { Todo } from './actions';

// Load todos from localStorage
const savedTodos = localStorage.getItem('htms-todos');
const todos: Todo[] = savedTodos ? JSON.parse(savedTodos) : [];

// Helper to compute filtered todos
function getFilteredTodos(todos: Todo[], filter: string): Todo[] {
  if (filter === 'active') return todos.filter(t => !t.done);
  if (filter === 'completed') return todos.filter(t => t.done);
  return todos;
}

// Initialize context
const initialContext = {
  todos,
  newTodoText: '',
  filter: 'all' as const,
  editingId: null,
  editingText: '',
  // Computed values
  filteredTodos: getFilteredTodos(todos, 'all'),
  activeCount: todos.filter(t => !t.done).length,
  completedCount: todos.filter(t => t.done).length
};

setContext(initialContext);

// Initialize events and router
initEvents();
router.init();
```

### styles.css

```css
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  min-height: 100vh;
  padding: 40px 20px;
}

.todo-app {
  max-width: 600px;
  margin: 0 auto;
  background: white;
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 30px;
  text-align: center;
}

.app-header h1 {
  font-size: 48px;
  font-weight: 200;
  letter-spacing: 2px;
}

.add-todo-form {
  display: flex;
  padding: 20px;
  border-bottom: 1px solid #eee;
}

.new-todo-input {
  flex: 1;
  padding: 12px 16px;
  font-size: 16px;
  border: 2px solid #eee;
  border-radius: 8px 0 0 8px;
  outline: none;
  transition: border-color 0.2s;
}

.new-todo-input:focus {
  border-color: #667eea;
}

.btn-add {
  padding: 12px 24px;
  background: #667eea;
  color: white;
  border: none;
  border-radius: 0 8px 8px 0;
  font-size: 16px;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-add:hover:not(:disabled) {
  background: #5568d3;
}

.btn-add:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.todos-container {
  padding: 20px;
}

.filters {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
}

.filter-btn {
  flex: 1;
  padding: 8px 16px;
  background: white;
  border: 2px solid #eee;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.filter-btn:hover {
  border-color: #667eea;
}

.filter-btn.active {
  background: #667eea;
  color: white;
  border-color: #667eea;
}

.todo-list {
  list-style: none;
}

.todo-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  border-bottom: 1px solid #eee;
  transition: background 0.2s;
}

.todo-item:hover {
  background: #f9f9f9;
}

.todo-item.completed .todo-text {
  text-decoration: line-through;
  opacity: 0.6;
}

.todo-content {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
}

.todo-checkbox {
  width: 20px;
  height: 20px;
  cursor: pointer;
}

.todo-text {
  flex: 1;
  cursor: pointer;
  user-select: none;
}

.todo-edit-input {
  flex: 1;
  padding: 8px;
  font-size: 16px;
  border: 2px solid #667eea;
  border-radius: 4px;
  outline: none;
}

.btn-delete {
  width: 32px;
  height: 32px;
  background: #ff4757;
  color: white;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s, background 0.2s;
}

.todo-item:hover .btn-delete {
  opacity: 1;
}

.btn-delete:hover {
  background: #ee5a6f;
}

.todo-stats {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 16px;
  margin-top: 16px;
  border-top: 1px solid #eee;
  font-size: 14px;
  color: #666;
}

.btn-clear {
  padding: 6px 12px;
  background: white;
  border: 1px solid #ff4757;
  color: #ff4757;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.btn-clear:hover {
  background: #ff4757;
  color: white;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #999;
}

.empty-state p {
  font-size: 18px;
}
```

### index.html

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>HTMS Todo App</title>
  <link rel="stylesheet" href="/src/styles.css">
</head>
<body>
  <div id="app"></div>
  <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

## Usage

1. **Install dependencies:**
```bash
npm install
```

2. **Compile HTMS:**
```bash
htms compile src/app.htms -o src/dist/
```

3. **Run dev server:**
```bash
npx vite
```

4. **Build for production:**
```bash
npx vite build
```

## Key Concepts Demonstrated

- ✅ Component composition (`TodoItem`, `TodoFilters`, `TodoStats`)
- ✅ Two-way data binding (`bind: ctx.newTodoText`)
- ✅ Event handling with parameters (`toggleTodo(todo.id)`)
- ✅ Conditional rendering (`@if ctx.todos.length > 0`)
- ✅ List rendering (`@each ctx.filteredTodos as todo`)
- ✅ Element directives (`@if(ctx.editingId == todo.id)`)
- ✅ Form handling (`onSubmit.prevent: addTodo`)
- ✅ LocalStorage persistence
- ✅ Computed values (filtered todos, counts)
- ✅ Keyboard handling (Enter, Escape in edit mode)
- ✅ Dynamic classes (`class: todo.done ? "completed" : ""`)

## Next Steps

- Add drag-and-drop reordering
- Add categories/tags
- Add due dates
- Add priority levels
- Add dark mode toggle
- Add animations with CSS transitions
