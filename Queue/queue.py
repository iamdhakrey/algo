class Task:
    def __init__(self, task_name, task_func, *args, **kwargs) -> None:
        self.task_name = task_name
        self.task_func = task_func
        self.args = args
        self.kwargs = kwargs

    def __str__(self) -> str:
        return f"{self.task_name}"


class Queue:
    def __init__(self):
        self.tasks_list = []

    def add_task(self, task):
        self.tasks_list.append(task)

    def get_task(self):
        return self.tasks_list.pop(0)

    def print(self):
        for task in self.tasks_list:
            print(task)


task1 = Task(task_name="add two number", task_func="task.add")
task2 = Task(task_name="sub two number", task_func="task.sub")
task3 = Task(task_name="multi two number", task_func="task.multi")

q = Queue()
q.add_task(task1)
q.add_task(task3)
q.add_task(task2)
q.print()
print(q.get_task())
