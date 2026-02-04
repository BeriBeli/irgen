use crate::error::Error;
use polars::prelude::*;

pub fn parse_register(df: DataFrame) -> Result<DataFrame, Error> {
    let parsed_df = df
        .lazy()
        // fullfill empty description
        .with_column(
            when(col("DESCRIPTION").is_null())
                .then(lit("No Description"))
                .otherwise(col("DESCRIPTION"))
                .alias("DESCRIPTION"),
        )
        // Unmerge cells and distribute content to each cell
        .select([col("*").fill_null_with_strategy(FillNullStrategy::Forward(None))])
        .with_columns(&[
            // caculate reg width by sum field width
            // "32"
            col("WIDTH")
                .cast(DataType::UInt32)
                .sum()
                .over(&[col("ADDR")])
                .cast(DataType::String)
                .alias("REG_WIDTH"),
            // reg width (bytes)
            (col("WIDTH")
                .cast(DataType::UInt32)
                .sum()
                .over(&[col("ADDR")])
                / lit(8))
            .alias("BYTES"),
            // reg's base name to parse "reg{n}, n=0~3"
            coalesce(&[col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"(.*?)\{n\}"), 1)])
            .alias("BASE_REG"),
            // if need explode "reg{n}, n=0~3" to reg_0, reg_1, reg_2, reg_3
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .contains(lit(r"\{n\}"), false)
                .alias("IS_EXPANDABLE"),
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"n\s*=\s*range\(([^)]+)\)"), 1)
                .str()
                .split(lit(","))
                .list()
                .eval(col("").str().strip_chars(lit(" \t")).cast(DataType::UInt32))
                .alias("ARGS"),
            // convert reg's offset to decimal
            col("ADDR")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit("0x([0-9a-fA-F]+)"), 1)
                .str()
                .to_integer(lit(16), Some(DataType::UInt32), false)
                .alias("BASE_ADDR"),
            // get field's bit offset
            col("BIT")
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"(?:\d+:)?(\d+)"), 1)
                .alias("BIT_OFFSET"),
        ])
        .with_column(
            when(col("ARGS").list().len().eq(lit(3)))
                .then(int_ranges(
                    col("ARGS").list().get(lit(0), true).cast(DataType::UInt32),
                    col("ARGS").list().get(lit(1), true).cast(DataType::UInt32),
                    col("ARGS").list().get(lit(2), true).cast(DataType::UInt32),
                    DataType::UInt32,
                ))
                .otherwise(
                    when(col("ARGS").list().len().eq(lit(2)))
                        .then(int_ranges(
                            col("ARGS").list().get(lit(0), true).cast(DataType::UInt32),
                            col("ARGS").list().get(lit(1), true).cast(DataType::UInt32),
                            lit(1),
                            DataType::UInt32,
                        ))
                        .otherwise(
                            when(col("ARGS").list().len().eq(lit(1)))
                                .then(int_ranges(
                                    lit(0),
                                    col("ARGS").list().get(lit(0), true).cast(DataType::UInt32),
                                    lit(1),
                                    DataType::UInt32,
                                ))
                                .otherwise(lit(Null {})),
                        ),
                )
                .alias("N_SERIES"),
        )
        .explode(by_name(["N_SERIES"], true))
        .filter(
            col("IS_EXPANDABLE")
                .and(col("N_SERIES").is_not_null())
                .or(col("IS_EXPANDABLE")
                    .not()
                    .and(col("FIELD").is_not_null())
                    .and(col("FIELD").neq(lit("")))),
        )
        .with_columns(&[
            when(col("IS_EXPANDABLE"))
                .then(
                    (col("BASE_ADDR").cast(DataType::UInt32)
                        + col("N_SERIES").cast(DataType::UInt32)
                            * col("BYTES").cast(DataType::UInt32))
                    .map(
                        |s| {
                            let ca = s.u32()?;
                            let new_ca: StringChunked = ca
                                .into_iter()
                                .map(|opt_x| opt_x.map(|x| format!("0x{:X}", x)))
                                .collect();
                            Ok(Some(new_ca.into_column()))
                        },
                        GetOutput::from_type(DataType::String),
                    ),
                )
                .otherwise(col("ADDR"))
                .alias("ADDR"),
            when(col("IS_EXPANDABLE"))
                .then(
                    col("BASE_REG").cast(DataType::String)
                        + lit("_")
                        + (col("N_SERIES")
                            .rank(
                                RankOptions {
                                    method: RankMethod::Dense,
                                    descending: false,
                                },
                                None,
                            )
                            .over([col("BASE_REG")])
                            - lit(1))
                        .cast(DataType::String),
                )
                .otherwise(col("REG"))
                .alias("REG"),
        ])
        .lazy()
        .group_by_stable(["REG"])
        .agg([
            col("ADDR").first(),
            col("REG_WIDTH").first(),
            col("FIELD"),
            // col("BIT"),
            col("WIDTH"),
            col("ATTRIBUTE"),
            // col("BYTES"),
            col("BIT_OFFSET"),
            col("DEFAULT"),
            col("DESCRIPTION"),
            // col("BASE_REG"),
            // col("IS_EXPANDABLE"),
            // col("BASE_ADDR"),
            col("N_SERIES"),
        ])
        .collect()?;

    Ok(parsed_df)
}
