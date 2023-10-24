import sqlite3


def connect_to_database():
    try:
        connection = sqlite3.connect("GroceryDB.db")
        print("Connected to SQLite database")
        return connection
    except sqlite3.Error as e:
        print(f"Error: {e}")

    finally:
        # Close the database connection
        if connection:
            connection.close()
            print("Connection closed")


def create_operation():
    try:
        conn = sqlite3.connect("newDB.db")
        cursor = conn.cursor()

        # create a table
        cursor.execute(
            """CREATE TABLE IF NOT EXISTS employees
                      (id INTEGER PRIMARY KEY,
                       name TEXT,
                       department TEXT)"""
        )

        # Commit the transaction
        conn.commit()

        print("Table created successfully!")

    except sqlite3.Error as e:
        print(f"Error: {e}")

    finally:
        # Close the database connection
        if conn:
            conn.close()
            print("Connection closed")


def read_operation():
    try:
        conn = sqlite3.connect("GroceryDB.db")
        cursor = conn.cursor()

        # Perform database operations here
        # Query the database for the top 5 rows of the GroceryDB table
        cursor.execute("SELECT * FROM GroceryDB LIMIT 5;")
        rows = cursor.fetchall()
        print("Data in the table:")
        for row in rows:
            print(row)

    except sqlite3.Error as e:
        print(f"Error: {e}")

    finally:
        # Close the database connection
        if conn:
            conn.close()
            print("Connection closed")


def update_operation():
    try:
        conn = sqlite3.connect("GroceryDB.db")
        cursor = conn.cursor()

        # Example data to be inserted
        data_to_insert = [
            (
                "general name",
                "count_products",
                "ingred_FPro",
                "avg_FPro_products",
                "avg_distance_root",
                "ingred_normalization_term",
                "semantic_tree_name",
                "semantic_tree_node",
            ),
            (
                "arabica coffee",
                21,
                0.18903204038025467,
                0.2754401549508692,
                2.0476190476190474,
                15.16666666666667,
                "",
                "",
            ),
            (
                "grape tomatoes",
                18,
                0.21119429773632484,
                0.4212998456790123,
                3.111111111111111,
                10.594047619047616,
                "",
                "",
            ),
            (
                "cherry tomatoes",
                11,
                0.23032828967178565,
                0.31386826599326595,
                2.1818181818181817,
                8.785714285714283,
                "",
                "",
            ),
            (
                "yellow bell pepper",
                16,
                0.23402118394914562,
                0.46428807870370364,
                4.375,
                8.323412698412698,
                "",
                "",
            ),
        ]

        # SQL query to insert data into the table

        cursor.executemany(
            """
            INSERT INTO GroceryDB 
            (general_name, count_products, ingred_FPro, avg_FPro_products, 
            avg_distance_root, ingred_normalization_term, semantic_tree_name, 
            semantic_tree_node)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?);
        """,
            data_to_insert,
        )

        # Commit the transaction
        conn.commit()

        print("Record updated successfully!")

    except sqlite3.Error as e:
        print(f"Error: {e}")

    finally:
        # Close the database connection
        if conn:
            conn.close()
            print("Connection closed")


def delete_operation():
    try:
        conn = sqlite3.connect("GroceryDB.db")
        cursor = conn.cursor()

        # Example data to be deleted
        data_to_delete = ("yellow bell pepper",)
        # SQL query to delete data from the table
        cursor.execute("DELETE FROM GroceryDB WHERE general_name=?;", data_to_delete)

        # Commit the transaction
        conn.commit()

        print("Record deleted successfully!")

    except sqlite3.Error as e:
        print(f"Error: {e}")

    finally:
        # Close the database connection
        if conn:
            conn.close()
            print("Connection closed")
