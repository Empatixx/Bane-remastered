package cz.empatix.assets.domain.metadata;


import com.google.common.base.Preconditions;

import java.util.List;

public record RawMetadata(
        int width,
        int height,
        int totalRows,
        int totalColumns,
        List<Row> rows

) {
    public RawMetadata {
        if(rows != null){
            Preconditions.checkArgument(totalRows > 0, "Total sprites must be greater than 0");
            Preconditions.checkArgument(totalColumns > 0, "Total columns must be greater than 0");
            Preconditions.checkArgument(rows.size() == totalRows, "Rows size must be equal to total sprites");
        } else {
            rows = List.of();
        }
        Preconditions.checkArgument(width > 0, "Width must be greater than 0");
        Preconditions.checkArgument(height > 0, "Height must be greater than 0");
    }

}
