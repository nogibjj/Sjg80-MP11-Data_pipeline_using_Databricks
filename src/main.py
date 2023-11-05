from pyspark.sql import SparkSession
from pyspark.sql.functions import avg, col,count, when
import sys
from io import StringIO
import contextlib

@contextlib.contextmanager
def capture_output():
    new_stdout, new_stderr = StringIO(), StringIO()
    old_stdout, old_stderr = sys.stdout, sys.stderr
    try:
        sys.stdout, sys.stderr = new_stdout, new_stderr
        yield sys.stdout, sys.stderr
    finally:
        sys.stdout, sys.stderr = old_stdout, old_stderr


def save_markdown(output, filename="output.md"):
    with open(filename, "w") as f:
        f.write("# PySpark Output\n\n")
        f.write("```\n")
        f.write(output)
        f.write("\n```\n")

def main():
    # Initialize a Spark session
    spark = SparkSession.builder \
        .appName("Obesity Data Analysis") \
        .master("local[*]") \
        .getOrCreate()
   
    # Load the data into a DataFrame
    df = spark.read.csv('data/ObesityDataSet.csv', header=True, inferSchema=True)

    # Filter the data where Age > 30 and Weight < 70
    filtered_data = df.filter((df.Age > 30) & (df.Weight < 70))

    # Calculate the percentage of people who smoke
    smoke_percentage = filtered_data.filter(df.SMOKE == 'yes').count() / filtered_data.count() * 100

    # Count the number of females and males
    gender_counts = df.groupBy("Gender").count()

    # Capture the output
    with capture_output() as (out, err):
        # Show the DataFrame schema to verify correct data types
        df.printSchema()

        # Show the percentage of people who smoke
        print(f"Percentage of people who smoke: {smoke_percentage}%")

        # Show the number of females and males
        gender_counts.show()

    # Save the output to markdown
    save_markdown(out.getvalue())

    # Stop the Spark session
    spark.stop()

if __name__ == '__main__':
    main()
