import unittest
from unittest.mock import patch
import main


class TestDatabaseConnection(unittest.TestCase):
    @patch("builtins.print")  # Mock the print function to capture output
    def test_connection(self, mock_print):

        # Call the connect_to_database function
        main.connect_to_database()

        # Assertions based on the expected output
        mock_print.assert_any_call("Connected to SQLite database")
        mock_print.assert_any_call("Connection closed")


class TestCreateOperation(unittest.TestCase):
    @patch("builtins.print")  # Mock the print function to capture output
    def test_create_operation(self, mock_print):
        # Call the create_operation function
        main.create_operation()

        # Assertions based on the expected output
        mock_print.assert_any_call("Table created successfully!")
        mock_print.assert_any_call("Connection closed")


class TestReadoperation(unittest.TestCase):
    @patch("builtins.print")  # Mock the print function to capture output
    def test_read_operation(self, mock_print):

        # Call the connect_to_database function
        main.read_operation()

        # Assertions based on the expected output
        mock_print.assert_any_call("Data in the table:")
        mock_print.assert_any_call("Connection closed")


class TestUpdateoperation(unittest.TestCase):
    @patch("builtins.print")  # Mock the print function to capture output
    def test_update_operation(self, mock_print):

        # Call the connect_to_database function
        main.update_operation()

        # Assertions based on the expected output
        mock_print.assert_any_call("Record updated successfully!")
        mock_print.assert_any_call("Connection closed")


class TestDeleteoperation(unittest.TestCase):
    @patch("builtins.print")  # Mock the print function to capture output
    def test_delete_operation(self, mock_print):

        # Call the connect_to_database function
        main.delete_operation()

        # Assertions based on the expected output
        mock_print.assert_any_call("Record deleted successfully!")
        mock_print.assert_any_call("Connection closed")


if __name__ == "__main__":
    unittest.main()
