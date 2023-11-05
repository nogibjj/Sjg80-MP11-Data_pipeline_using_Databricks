# Sjg80-PySpark Data Processing

[![CI](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/actions/workflows/PySpark.yml/badge.svg)](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/actions/workflows/PySpark.yml)

# What is PySpark?
   - PySpark is the Python library for Apache Spark. It provides an interface to work with Spark using Python. PySpark allows you to leverage the capabilities of Spark for distributed data processing while writing your code in Python, a popular and versatile programming language.
   - It includes various libraries and modules that facilitate working with distributed data and performing tasks like data manipulation, analysis, and machine learning at scale.
   - PySpark supports the distributed processing of data across a cluster of computers, making it well-suited for big data and parallel processing tasks.
   - It provides an easy-to-use API for data processing tasks and integrates with other Python libraries, making it a valuable tool for data scientists and engineers.

![image](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/assets/142270941/f199a35d-67e1-4cbc-881a-1e59af74660a)

# What is Spark SQL?
   - Spark SQL is a component of Apache Spark that provides a programming interface to work with structured and semi-structured data using SQL queries. It allows you to query structured data in Spark programs.
   - Spark SQL extends Spark's capabilities to work with structured data formats like Parquet, Avro, ORC, and JSON, as well as to connect to various data sources, including Hive, HBase, and relational databases.
   - It introduces a DataFrame API, which is similar to working with tables in a relational database. DataFrames are distributed collections of data organized into named columns, which makes them suitable for both data exploration and querying.
   - With Spark SQL, you can seamlessly mix SQL queries with your PySpark or Scala code, enabling complex data processing and analysis tasks using both SQL and programmatic approaches.
   - Spark SQL optimizes query execution by pushing down query processing to the underlying storage and minimizing data movement across the cluster. This makes it an efficient choice for working with large datasets.

In summary, PySpark is the Python library for Apache Spark, which allows you to write Spark applications in Python. Spark SQL is a part of Apache Spark that facilitates working with structured data using SQL queries and DataFrames, making it easier to process, analyze, and query structured data within the Spark framework. Both PySpark and Spark SQL are valuable tools in the big data and data analytics domain.


# PySpark Obesity Data Analysis

This Python script is an example of using PySpark, a Python library for Apache Spark, to perform data analysis on a dataset related to obesity. The script loads the dataset, performs data filtering and aggregation, and captures the output for further analysis.. The dataset used is `ObesityDataSet.csv`.

## Dataset

The dataset contains the following columns:

- Gender
- Age
- Height
- Weight
- family_history_with_overweight
- FAVC
- FCVC
- NCP
- CAEC
- SMOKE
- CH2O
- SCC
- FAF
- TUE
- CALC
- MTRANS
- NObeyesdad


## Prerequisites
Before running this code, you need to have the following installed:

1.- Python: Make sure you have Python installed on your system.

2.- PySpark: PySpark is the Python library for Apache Spark. You need to install it. You can install PySpark using pip:

```bash
pip install pyspark
```
3.- Dataset: The script assumes you have a dataset named "ObesityDataSet.csv" in a "data" directory. You should modify the dataset path accordingly.

## Code Explanation
Imports and Function Definitions
The script starts with the necessary imports, including PySpark, and defines some utility functions.
•	capture_output: This context manager captures standard output and standard error during code execution.
•	save_markdown: This function saves captured output to a Markdown file.

## Main Function
The main function performs the following steps:
1.	Initialize a Spark Session: It sets up a Spark session to work with Spark.
2.	Load Data: The script reads the dataset ("ObesityDataSet.csv") into a Spark DataFrame and registers it as a temporary SQL view.
3.	Data Filtering and Aggregation with Spark SQL:
•	It uses Spark SQL to filter data where Age is greater than 30 and Weight is less than 70.
•	It calculates the percentage of people who smoke and the average age, height, and weight.
4.	Data Filtering and Aggregation with DataFrame Operations:
•	It performs the same filtering and aggregation operations using DataFrame operations.
•	It also counts the number of females and males in the dataset.
5.	Capture Output: The script captures the output, which includes the DataFrame schema, the percentage of people who smoke, and the number of females and males.
6.	Save Output: It saves the captured output to a Markdown file named "output.md."
7.	Stop Spark Session: The Spark session is stopped to release resources.

## Analysis

The code performs the following operations:

1. Filters the data to include only those records where Age > 30 and Weight < 70.
2. Calculates the percentage of these people who smoke.
3. Counts the number of females and males in the entire dataset.

## Usage

Ensure that you have set up Spark and have the dataset available. The script will produce output and save it to "output.md" for further analysis.
To run the code, simply execute the Python script:

```bash
1. Open codespaces
2. Wait for environment to be installed
3. Run: pip install pyspark
3. Run: python main.py
```
![image](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/assets/142270941/d04d480b-9aaf-4567-94ec-ad12607b6d8a)

Sucessfull jobs on Spark environment:

![image](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/assets/142270941/8a71a324-67c7-4e6b-b695-8d93a35823fe)

## Results 
The results will be printed to the console and saved to a markdown file named output.md.
