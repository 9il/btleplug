package com.nonpolynomial.btleplug.android.impl;

import java.util.Arrays;

public class ScanFilter {
    private final String[] uuids;
    /** Android ScanSettings.SCAN_MODE_* (0=BALANCED, 1=LOW_LATENCY, 2=LOW_POWER, 3=OPPORTUNISTIC). */
    private final int scanMode;

    public ScanFilter(String uuids[], int scanMode) {
        if (uuids == null) {
            this.uuids = new String[0];
        } else {
            int len = uuids.length;
            this.uuids = Arrays.copyOf(uuids, len);
        }
        this.scanMode = scanMode;
    }

    public String[] getUuids() {
        int len = uuids.length;
        return Arrays.copyOf(uuids, len);
    }

    public int getScanMode() {
        return this.scanMode;
    }
}
