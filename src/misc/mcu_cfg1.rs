#[doc = "Register `mcu_cfg1` reader"]
pub struct R(crate::R<MCU_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCU_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCU_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCU_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mcu_cfg1` writer"]
pub struct W(crate::W<MCU_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCU_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCU_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCU_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mcu1_dfs_req` reader - "]
pub type REG_MCU1_DFS_REQ_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_dfs_req` writer - "]
pub type REG_MCU1_DFS_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_1` reader - "]
pub type RESERVED_1_R = crate::BitReader<bool>;
#[doc = "Field `sts_mcu1_dfs_ack` reader - "]
pub type STS_MCU1_DFS_ACK_R = crate::BitReader<bool>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu1_srst_en` reader - "]
pub type REG_MCU1_SRST_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mcu1_srst_en` writer - "]
pub type REG_MCU1_SRST_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MCU_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_6_9` reader - "]
pub type RESERVED_6_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_mcu1_lpmd_b` reader - "]
pub type STS_MCU1_LPMD_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_12_15` reader - "]
pub type RESERVED_12_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCU1_WFI_FORCE` reader - "]
pub type MCU1_WFI_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `MCU1_WFI_FORCE` writer - "]
pub type MCU1_WFI_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_17_27` reader - "]
pub type RESERVED_17_27_R = crate::FieldReader<u16, u16>;
#[doc = "Field `mcu1_ndm_rstn_en` reader - "]
pub type MCU1_NDM_RSTN_EN_R = crate::BitReader<bool>;
#[doc = "Field `mcu1_ndm_rstn_en` writer - "]
pub type MCU1_NDM_RSTN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_CFG1_SPEC, bool, O>;
#[doc = "Field `mcu1_hart_rstn_en` reader - "]
pub type MCU1_HART_RSTN_EN_R = crate::BitReader<bool>;
#[doc = "Field `mcu1_hart_rstn_en` writer - "]
pub type MCU1_HART_RSTN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCU_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_30_31` reader - "]
pub type RESERVED_30_31_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mcu1_dfs_req(&self) -> REG_MCU1_DFS_REQ_R {
        REG_MCU1_DFS_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reserved_1(&self) -> RESERVED_1_R {
        RESERVED_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_mcu1_dfs_ack(&self) -> STS_MCU1_DFS_ACK_R {
        STS_MCU1_DFS_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_mcu1_srst_en(&self) -> REG_MCU1_SRST_EN_R {
        REG_MCU1_SRST_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9"]
    #[inline(always)]
    pub fn reserved_6_9(&self) -> RESERVED_6_9_R {
        RESERVED_6_9_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sts_mcu1_lpmd_b(&self) -> STS_MCU1_LPMD_B_R {
        STS_MCU1_LPMD_B_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reserved_12_15(&self) -> RESERVED_12_15_R {
        RESERVED_12_15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mcu1_wfi_force(&self) -> MCU1_WFI_FORCE_R {
        MCU1_WFI_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27"]
    #[inline(always)]
    pub fn reserved_17_27(&self) -> RESERVED_17_27_R {
        RESERVED_17_27_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_en(&self) -> MCU1_NDM_RSTN_EN_R {
        MCU1_NDM_RSTN_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn mcu1_hart_rstn_en(&self) -> MCU1_HART_RSTN_EN_R {
        MCU1_HART_RSTN_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reserved_30_31(&self) -> RESERVED_30_31_R {
        RESERVED_30_31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_dfs_req(&mut self) -> REG_MCU1_DFS_REQ_W<0> {
        REG_MCU1_DFS_REQ_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_srst_en(&mut self) -> REG_MCU1_SRST_EN_W<4> {
        REG_MCU1_SRST_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_wfi_force(&mut self) -> MCU1_WFI_FORCE_W<16> {
        MCU1_WFI_FORCE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_ndm_rstn_en(&mut self) -> MCU1_NDM_RSTN_EN_W<28> {
        MCU1_NDM_RSTN_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_hart_rstn_en(&mut self) -> MCU1_HART_RSTN_EN_W<29> {
        MCU1_HART_RSTN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mcu_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcu_cfg1](index.html) module"]
pub struct MCU_CFG1_SPEC;
impl crate::RegisterSpec for MCU_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcu_cfg1::R](R) reader structure"]
impl crate::Readable for MCU_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcu_cfg1::W](W) writer structure"]
impl crate::Writable for MCU_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_cfg1 to value 0x30"]
impl crate::Resettable for MCU_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
