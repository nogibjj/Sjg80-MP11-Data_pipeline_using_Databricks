# Sjg80-Data pipeline using the Databricks platform.

[![CI](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/actions/workflows/PySpark.yml/badge.svg)](https://github.com/nogibjj/Sjg80-Mini-Project10-PySpark-Data-Processing/actions/workflows/PySpark.yml)

# Databricks Data Pipeline Guide

## Week 11: Data Pipeline with Databricks

### Requirements

Create a data pipeline using Databricks that includes at least one data source and one data sink.

Build an end-to-end data pipeline in Databricks, encompassing the ingestion of raw data, transformation, and subsequent analysis on the processed data.

**Note:**
While this guide demonstrates the creation of a complete data pipeline using Databricks notebooks and a Databricks job for workflow orchestration, it's recommended to use Delta Live Tables for building reliable, maintainable, and testable data processing pipelines.

### What is a Data Pipeline?

A data pipeline implements the steps required to move data from source systems, transform the data based on requirements, and store the data in a target system. It includes all processes necessary to turn raw data into prepared data for user consumption. Commonly, an Extract, Transform, and Load (ETL) workflow exemplifies a data pipeline.

### Data Pipeline Steps

This guide helps you build data pipelines on Databricks with the following example:

1. **Use Databricks Features to Explore a Raw Dataset.**

2. **Create a Databricks Notebook to Ingest Raw Source Data:**
   - Ingest raw data into a target table.

3. **Create a Databricks Notebook to Transform Raw Source Data:**
   - Transform raw data and write it to a target table.

4. **Create a Databricks Notebook to Query the Transformed Data:**
   - Execute queries on the transformed data.

5. **Automate the Data Pipeline with a Databricks Job.**

### Requirements

- You are logged into Databricks and in the Data Science & Engineering workspace.
- You have permission to create a cluster or access to a cluster.
- (Optional) To publish tables to Unity Catalog, create a catalog and schema in Unity Catalog.

### Example: Million Song Dataset

The dataset used is a subset of the Million Song Dataset, available in the sample datasets included in your Databricks workspace.

### Step-by-Step Guide

#### Step 1: Create a Cluster

Create a cluster to provide the compute resources needed to run commands.

1. Click on "Compute" in the sidebar.
2. On the Compute page, click "Create Cluster."
3. Enter a unique name for the cluster.
4. In Access mode, select "Single User."
5. In Single user or service principal access, select your user name.
6. Leave the remaining values in their default state and click "Create Cluster."

#### Step 2: Explore the Source Data

Use Databricks features to explore the raw dataset.
<img width="1509" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/b7f65405-82e9-4231-bb18-55035839647f">

<img width="1724" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/eaab81df-9da9-4419-8cd8-97f3f15cca57">

#### Step 3: Ingest the Raw Data

Load the raw data into a table for further processing.

<img width="1679" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/7a11d845-76f1-4864-bc51-9edb6134a9ff">

#### Step 4: Prepare the Raw Data

Transform the raw songs data by filtering out unneeded columns and adding a new field containing a timestamp for the creation of the new record.

<img width="1677" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/35014441-c00e-459b-8dac-1cc79c6ad8d7">

#### Step 5: Query the Transformed Data

Extend the processing pipeline by adding queries to analyze the songs data using the prepared records.

<img width="1687" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/0a929e1f-2927-41e0-ab31-6e8f3fd523e3">

#### Step 6: Create a Databricks Job to Run the Pipeline

Automate the data ingestion, processing, and analysis steps using a Databricks job.

<img width="1175" alt="image" src="https://github.com/nogibjj/Sjg80-MP11-Data_pipeline_using_Databricks/assets/142270941/fd77dad3-b047-4ed9-af43-3b642543fecd">

## Running the Data Pipeline

To execute the script, you need to run each notebook in sequential order. Ensure that you follow the setup and configuration instructions outlined in the documentation.

1. Run the notebook for ingesting raw data.
2. Run the notebook for transforming raw data.
3. Run the notebook for querying the transformed data.

For automation, create and run a Databricks job, providing a streamlined workflow for the end-to-end data processing.

## References

* [What is a data pipeline?](https://docs.databricks.com/en/getting-started/data-pipeline-get-started.html)
