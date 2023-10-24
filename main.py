from mylib.lib import connect_to_database,create_operation,read_operation,update_operation,delete_operation
import time
import psutil

def main():
    start_time = time.time()
    memory_before = psutil.virtual_memory().used / (1024.0 ** 2)  # Convert bytes to MB

    connect_to_database()
    create_operation()
    read_operation()
    update_operation()
    delete_operation()

    end_time = time.time()
    memory_after = psutil.virtual_memory().used / (1024.0 ** 2)
    
    elapsed_time = end_time - start_time
    memory_used = memory_after - memory_before
    print(elapsed_time)
    print(memory_used)
    
    with open("python_performance.md", "w", encoding="utf-8") as file:
        file.write("## Python Performance Report\n")
        file.write(f"- Time taken: {elapsed_time} seconds\n")
        file.write(f"- Memory used: {memory_used} MB\n")
        file.write("### Operations Performed\n")
        file.write("1. connect to database\n")
        file.write("2. create a table\n")
        file.write("3. read a table\n")
        file.write("4. updata a table\n")
        file.write("5. delete a table\n")

    return 1




if __name__ == "__main__":
    main()