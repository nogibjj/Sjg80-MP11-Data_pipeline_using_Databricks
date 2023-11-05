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

This project uses PySpark to analyze a dataset containing information about individuals and their obesity levels. The dataset used is `ObesityDataSet.csv`.

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

## Analysis

The code performs the following operations:

1. Filters the data to include only those records where Age > 30 and Weight < 70.
2. Calculates the percentage of these people who smoke.
3. Counts the number of females and males in the entire dataset.

## Usage

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
